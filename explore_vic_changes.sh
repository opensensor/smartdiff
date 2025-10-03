#!/bin/bash
# Script to explore VIC register changes between versions

COMPARISON_ID="2ba41148-9ca6-4614-96ae-6f03ae90ab32"

echo "=== VIC Register Changes Explorer ==="
echo ""
echo "Comparison ID: $COMPARISON_ID"
echo "Source: /home/matteius/isp-was-better/driver/tx_isp_vic.c"
echo "Target: /home/matteius/isp-latest/driver/tx_isp_vic.c"
echo ""

# Function to get diff for a specific function
get_function_diff() {
    local func_name="$1"
    echo "=== Function: $func_name ==="
    echo ""
    
    curl -s -X POST http://127.0.0.1:8011/message \
      -H "Content-Type: application/json" \
      -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{\"name\":\"get_function_diff\",\"arguments\":{\"comparison_id\":\"$COMPARISON_ID\",\"function_name\":\"$func_name\",\"include_content\":true}}}" \
      | jq -r '.result.content[0].text'
    
    echo ""
    echo "---"
    echo ""
}

# Check command line argument
if [ $# -eq 1 ]; then
    # Get diff for specific function
    get_function_diff "$1"
    exit 0
fi

# Interactive menu
echo "Top Modified VIC Functions:"
echo ""
echo "1. vic_framedone_irq_function (6% changed) - Frame done interrupt handler"
echo "2. vic_mdma_irq_function (12% changed) - MDMA interrupt handler"
echo "3. ispvic_frame_channel_clearbuf (19% changed) - Buffer management"
echo "4. isp_vic_cmd_set (14% changed) - Command processing"
echo "5. vic_proc_write (10% changed) - Proc interface"
echo "6. tx_isp_vic_hw_init (3% changed) - Hardware initialization"
echo "7. tx_vic_enable_irq - Interrupt enable"
echo "8. tx_vic_disable_irq - Interrupt disable"
echo "9. vic_core_s_stream (2% changed) - Stream control"
echo "10. tx_isp_vic_apply_full_config (2% changed) - Full configuration"
echo ""
echo "11. List all changed functions"
echo "12. Get comparison summary"
echo "13. Search for specific register writes"
echo ""
echo "0. Exit"
echo ""

read -p "Select option (0-13): " choice

case $choice in
    1)
        get_function_diff "vic_framedone_irq_function"
        ;;
    2)
        get_function_diff "vic_mdma_irq_function"
        ;;
    3)
        get_function_diff "ispvic_frame_channel_clearbuf"
        ;;
    4)
        get_function_diff "isp_vic_cmd_set"
        ;;
    5)
        get_function_diff "vic_proc_write"
        ;;
    6)
        get_function_diff "tx_isp_vic_hw_init"
        ;;
    7)
        get_function_diff "tx_vic_enable_irq"
        ;;
    8)
        get_function_diff "tx_vic_disable_irq"
        ;;
    9)
        get_function_diff "vic_core_s_stream"
        ;;
    10)
        get_function_diff "tx_isp_vic_apply_full_config"
        ;;
    11)
        echo "=== All Changed Functions ==="
        echo ""
        curl -s -X POST http://127.0.0.1:8011/message \
          -H "Content-Type: application/json" \
          -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{\"name\":\"list_changed_functions\",\"arguments\":{\"comparison_id\":\"$COMPARISON_ID\",\"limit\":100}}}" \
          | jq -r '.result.content[0].text'
        ;;
    12)
        echo "=== Comparison Summary ==="
        echo ""
        curl -s -X POST http://127.0.0.1:8011/message \
          -H "Content-Type: application/json" \
          -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{\"name\":\"get_comparison_summary\",\"arguments\":{\"comparison_id\":\"$COMPARISON_ID\"}}}" \
          | jq -r '.result.content[0].text'
        ;;
    13)
        read -p "Enter register offset (e.g., 0x300): " reg_offset
        echo ""
        echo "=== Searching for writes to register $reg_offset ==="
        echo ""
        echo "In WAS-BETTER:"
        grep -n "writel.*$reg_offset" /home/matteius/isp-was-better/driver/tx_isp_vic.c | head -10
        echo ""
        echo "In LATEST:"
        grep -n "writel.*$reg_offset" /home/matteius/isp-latest/driver/tx_isp_vic.c | head -10
        ;;
    0)
        echo "Exiting..."
        exit 0
        ;;
    *)
        echo "Invalid option"
        exit 1
        ;;
esac

echo ""
echo "=== Done ==="
echo ""
echo "To get diff for any function, run:"
echo "  $0 FUNCTION_NAME"
echo ""
echo "Example:"
echo "  $0 vic_framedone_irq_function"

