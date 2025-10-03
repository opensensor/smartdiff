# VIC (Video Input Controller) Register Analysis

## Summary

Analysis of VIC control register writes between the "was-better" and "latest" ISP driver versions.

**Key File:** `tx_isp_vic.c`

## Critical VIC Register Writes Found

### 1. VIC Register 0x300 - Buffer Index Control

**Location:** `vic_framedone_irq_function()` - Frame Done Interrupt Handler

**WAS-BETTER Version:**
```c
// CRITICAL FIX: Preserve EXACT control bits 0x80000020 when updating buffer index
if (vic_regs) {
    u32 reg_val = readl(vic_regs + 0x300);
    // PRESERVE EXACT control bits (0x80000020) and only update buffer index in bits 16-19
    reg_val = (reg_val & 0xfff0ffff) | shifted_value;  // Clear bits 16-19, set new buffer index
    
    // FORCE control bits if they were lost
    if ((reg_val & 0x80000020) != 0x80000020) {
        reg_val |= 0x80000020;  // Force control bits back on
        pr_warn("*** VIC FRAME DONE: FORCED control bits 0x80000020 back on! ***\n");
    }
    
    writel(reg_val, vic_regs + 0x300);
}
```

**Key Insight:** The was-better version **preserves control bits 0x80000020** when updating the buffer index.

### 2. VIC Interrupt Mask Register 0x1e8

**Location:** `tx_vic_enable_irq()` - Interrupt Enable Function

**WAS-BETTER Version:**
```c
writel(0xFFFFFFFE, vic_dev->vic_regs + 0x1e8);  /* Enable frame done interrupt */
```

**LATEST Version:**
```c
// Similar but with different logging
writel(0xFFFFFFFE, vic_regs + 0x1e8);  /* MainMask=0xFFFFFFFE (bit 0=0 ENABLES frame-done) */
```

**Key Insight:** Mask register uses **inverted logic** - bit 0=0 means ENABLED

### 3. VIC Interrupt Status Registers

**Location:** `tx_vic_enable_irq()` - Interrupt Enable Function

**Both Versions:**
```c
writel(0xFFFFFFFF, vic_dev->vic_regs + 0x1f0);  /* Clear main interrupt status */
writel(0xFFFFFFFF, vic_dev->vic_regs + 0x1f4);  /* Clear MDMA interrupt status */
```

### 4. VIC Control Registers - Hardware Configuration

**Location:** `tx_isp_vic_apply_full_config()` - Full VIC Configuration

**LATEST Version:**
```c
writel(0x2d0, vic_regs + 0x100);        /* Interrupt configuration */
writel(0x2b, vic_regs + 0x14);          /* Interrupt control - reference driver value */
writel(0x800800, vic_regs + 0x60);      /* Control register */
writel(0x9d09d0, vic_regs + 0x64);      /* Control register */
writel(0x6002, vic_regs + 0x70);        /* Control register */
```

## VIC Register Map

Based on the code analysis:

| Offset | Name | Purpose | Key Values |
|--------|------|---------|------------|
| 0x0 | VIC_CTRL | Main control | 0x1 = enable, 0x0 = disable |
| 0x4 | VIC_STATUS | Status register | Read-only |
| 0xc | Mode/Format | MIPI mode setting | 0x3 = MIPI mode |
| 0x14 | INT_CONTROL | Interrupt control | 0x2b (reference value) |
| 0x60 | CONTROL_1 | Control register 1 | 0x800800 |
| 0x64 | CONTROL_2 | Control register 2 | 0x9d09d0 |
| 0x70 | CONTROL_3 | Control register 3 | 0x6002 |
| 0x100 | INT_CONFIG | Interrupt config | 0x2d0 |
| 0x300 | BUFFER_INDEX | Buffer index + control bits | bits 16-19 = buffer index, 0x80000020 = control bits |
| 0x380 | CURRENT_FRAME | Current frame address | Read-only |
| 0x1e8 | INT_MASK_MAIN | Main interrupt mask | 0xFFFFFFFE = enable frame-done (inverted logic) |
| 0x1ec | INT_MASK_MDMA | MDMA interrupt mask | Various |
| 0x1f0 | INT_STATUS_MAIN | Main interrupt status | Write 0xFFFFFFFF to clear |
| 0x1f4 | INT_STATUS_MDMA | MDMA interrupt status | Write 0xFFFFFFFF to clear |

## Key Differences Between Versions

### Modified Functions (Most Changed)

1. **`ispvic_frame_channel_clearbuf`** - 19% changed
   - Buffer management changes
   
2. **`isp_vic_cmd_set`** - 14% changed
   - Command processing changes
   
3. **`vic_mdma_irq_function`** - 12% changed
   - MDMA interrupt handling changes
   
4. **`vic_framedone_irq_function`** - 6% changed
   - **Critical:** Control bit preservation logic for register 0x300

### Added Functions (8 new)

The latest version added 8 new functions (need to list them separately)

### Deleted Functions (16 removed)

The was-better version had 16 functions that were removed in latest

## Critical Finding: Register 0x300 Control Bits

The **most important difference** is in how register 0x300 is handled:

**WAS-BETTER approach:**
- Reads current value
- Preserves control bits 0x80000020
- Only updates buffer index in bits 16-19
- Forces control bits back on if they were lost
- Logs when control bits are forced

**Potential LATEST issue:**
- May not preserve control bits correctly
- Could explain why VIC interrupts stop working

## VIC Base Addresses

**Primary VIC registers:** 0x133e0000 (main control)
**Secondary VIC registers:** 0x10023000 (alternate/control space)

Both versions map these two register spaces.

## Interrupt Flow

1. **Frame Done Interrupt** triggers `vic_framedone_irq_function()`
2. Function reads VIC register 0x380 to get current frame address
3. Searches buffer list to find matching buffer
4. Updates register 0x300 with buffer index
5. **CRITICAL:** Must preserve control bits 0x80000020 in register 0x300
6. Hardware should automatically trigger ISP core interrupts

## Recommendations

1. **Verify register 0x300 handling** in the latest version
2. **Check if control bits 0x80000020 are preserved** during buffer index updates
3. **Compare interrupt enable sequences** between versions
4. **Verify MDMA interrupt handling** (12% changed)
5. **Review buffer management** in `ispvic_frame_channel_clearbuf` (19% changed)

## Next Steps

To get the complete diff of any function:
```bash
# Get diff for a specific function
curl -s -X POST http://127.0.0.1:8011/message \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"get_function_diff","arguments":{"comparison_id":"2ba41148-9ca6-4614-96ae-6f03ae90ab32","function_name":"FUNCTION_NAME","include_content":true}}}' \
  | jq -r '.result.content[0].text'
```

Replace `FUNCTION_NAME` with any of the modified functions listed above.

