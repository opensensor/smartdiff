#!/bin/bash
# Search for VIC (Vectored Interrupt Controller) register writes in ISP driver

echo "=== Searching for VIC Control Register Writes ==="
echo ""

WAS_BETTER="/home/matteius/isp-was-better/driver"
LATEST="/home/matteius/isp-latest/driver"

# Common VIC register patterns
VIC_PATTERNS=(
    "VIC.*CTRL"
    "VIC.*CONTROL"
    "VIC.*ENABLE"
    "VIC.*DISABLE"
    "VIC.*MASK"
    "VIC.*STATUS"
    "VIC.*PRIORITY"
    "VIC.*VECTOR"
    "VIC.*IRQ"
    "VIC.*INT"
    "vic_.*write"
    "vic_.*reg"
    "writel.*vic"
    "iowrite.*vic"
    "0x[0-9a-fA-F]+.*vic"  # Hex addresses with vic
)

echo "1. Searching in WAS-BETTER version..."
echo "   Path: $WAS_BETTER"
echo ""

for pattern in "${VIC_PATTERNS[@]}"; do
    results=$(grep -rni "$pattern" "$WAS_BETTER" --include="*.c" --include="*.h" 2>/dev/null | head -5)
    if [ -n "$results" ]; then
        echo "   Pattern: $pattern"
        echo "$results" | sed 's/^/      /'
        echo ""
    fi
done

echo ""
echo "2. Searching in LATEST version..."
echo "   Path: $LATEST"
echo ""

for pattern in "${VIC_PATTERNS[@]}"; do
    results=$(grep -rni "$pattern" "$LATEST" --include="*.c" --include="*.h" 2>/dev/null | head -5)
    if [ -n "$results" ]; then
        echo "   Pattern: $pattern"
        echo "$results" | sed 's/^/      /'
        echo ""
    fi
done

echo ""
echo "3. Searching for generic interrupt controller writes..."
echo ""

# Generic interrupt patterns
INT_PATTERNS=(
    "writel.*0x[0-9a-fA-F]+.*IRQ"
    "writel.*0x[0-9a-fA-F]+.*INT"
    "iowrite.*IRQ"
    "iowrite.*INT"
    "system_reg_write.*IRQ"
    "system_reg_write.*INT"
)

echo "   In WAS-BETTER:"
for pattern in "${INT_PATTERNS[@]}"; do
    results=$(grep -rni "$pattern" "$WAS_BETTER" --include="*.c" 2>/dev/null | head -3)
    if [ -n "$results" ]; then
        echo "      Pattern: $pattern"
        echo "$results" | sed 's/^/         /'
    fi
done

echo ""
echo "   In LATEST:"
for pattern in "${INT_PATTERNS[@]}"; do
    results=$(grep -rni "$pattern" "$LATEST" --include="*.c" 2>/dev/null | head -3)
    if [ -n "$results" ]; then
        echo "      Pattern: $pattern"
        echo "$results" | sed 's/^/         /'
    fi
done

echo ""
echo "4. Searching for register base addresses and offsets..."
echo ""

# Look for register definitions
echo "   Register definitions in headers:"
grep -rn "define.*VIC" "$WAS_BETTER/include" "$LATEST/include" 2>/dev/null | head -10

echo ""
echo "5. Searching for interrupt enable/disable functions..."
echo ""

# Functions that might control interrupts
FUNC_PATTERNS=(
    "enable.*irq"
    "disable.*irq"
    "mask.*irq"
    "unmask.*irq"
    "enable.*interrupt"
    "disable.*interrupt"
)

echo "   Functions in WAS-BETTER:"
for pattern in "${FUNC_PATTERNS[@]}"; do
    results=$(grep -rn "^[a-z_].*$pattern.*(" "$WAS_BETTER" --include="*.c" 2>/dev/null | head -2)
    if [ -n "$results" ]; then
        echo "$results" | sed 's/^/      /'
    fi
done

echo ""
echo "   Functions in LATEST:"
for pattern in "${FUNC_PATTERNS[@]}"; do
    results=$(grep -rn "^[a-z_].*$pattern.*(" "$LATEST" --include="*.c" 2>/dev/null | head -2)
    if [ -n "$results" ]; then
        echo "$results" | sed 's/^/      /'
    fi
done

echo ""
echo "=== Search Complete ==="

