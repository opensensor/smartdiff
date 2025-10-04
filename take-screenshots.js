#!/usr/bin/env node

const puppeteer = require('puppeteer');
const path = require('path');
const fs = require('fs');

// Helper function to wait
const wait = (ms) => new Promise(resolve => setTimeout(resolve, ms));

const SCREENSHOTS_DIR = path.join(__dirname, 'screenshots');
const BASE_URL = 'http://localhost:3000';
const SOURCE_DIR = '/home/matteius/isp-was-better/driver';
const TARGET_DIR = '/home/matteius/isp-latest/driver';

// Ensure screenshots directory exists
if (!fs.existsSync(SCREENSHOTS_DIR)) {
  fs.mkdirSync(SCREENSHOTS_DIR, { recursive: true });
}

async function takeScreenshots() {
  console.log('🚀 Starting Smart Diff screenshot capture...\n');

  const browser = await puppeteer.launch({
    headless: false, // Set to false to see what's happening
    defaultViewport: {
      width: 1920,
      height: 1080
    },
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--window-size=1920,1080'
    ]
  });

  try {
    const page = await browser.newPage();
    
    // Set localStorage before navigating
    await page.evaluateOnNewDocument((source, target) => {
      localStorage.setItem('smartdiff-source-directory', source);
      localStorage.setItem('smartdiff-target-directory', target);
    }, SOURCE_DIR, TARGET_DIR);

    console.log('📂 Navigating to Smart Diff...');
    await page.goto(BASE_URL, { waitUntil: 'networkidle2', timeout: 30000 });

    // Wait for the page to load
    await wait(2000);

    // Fix CSS overflow issues
    await page.evaluate(() => {
      // Add CSS to prevent horizontal overflow
      const style = document.createElement('style');
      style.textContent = `
        body, html {
          overflow-x: hidden !important;
          max-width: 100vw !important;
        }
        * {
          max-width: 100% !important;
        }
      `;
      document.head.appendChild(style);
    });

    console.log('✅ Page loaded successfully');
    console.log(`📍 Source: ${SOURCE_DIR}`);
    console.log(`📍 Target: ${TARGET_DIR}\n`);

    // Fill in the directory inputs
    console.log('📝 Filling in directory paths...');

    // Find and fill source directory
    const sourceInputs = await page.$$('input[type="text"]');
    if (sourceInputs.length >= 2) {
      await sourceInputs[0].click({ clickCount: 3 }); // Select all
      await sourceInputs[0].type(SOURCE_DIR);

      await sourceInputs[1].click({ clickCount: 3 }); // Select all
      await sourceInputs[1].type(TARGET_DIR);
    }

    await wait(1000);

    // Click the Start Comparison button
    console.log('🔄 Starting comparison...');
    const buttons = await page.$$('button');
    let startButton = null;

    for (const button of buttons) {
      const text = await page.evaluate(el => el.textContent, button);
      if (text.includes('Start Comparison') || text.includes('▶️')) {
        startButton = button;
        break;
      }
    }

    if (startButton) {
      await startButton.click();
    } else {
      throw new Error('Could not find Start Comparison button');
    }

    // Wait for comparison to complete - look for results
    console.log('⏳ Waiting for comparison to complete (this may take a while)...');
    console.log('   Checking every 5 seconds for completion...');

    // Poll for completion - look for result indicators
    let completed = false;
    let attempts = 0;
    const maxAttempts = 60; // 5 minutes max

    while (!completed && attempts < maxAttempts) {
      await wait(5000);
      attempts++;

      // Check if comparison is done by looking for result content
      const hasResults = await page.evaluate(() => {
        const text = document.body.textContent;
        return text.includes('Total Functions') ||
               text.includes('Changed Functions') ||
               text.includes('Similarity') ||
               text.includes('functions analyzed');
      });

      if (hasResults) {
        completed = true;
        console.log(`   ✓ Comparison completed after ${attempts * 5} seconds`);
      } else {
        console.log(`   ... still waiting (${attempts * 5}s elapsed)`);
      }
    }

    if (!completed) {
      console.log('   ⚠️  Timeout reached, proceeding anyway...');
    }

    await wait(2000); // Extra wait for UI to stabilize

    console.log('✅ Comparison should be complete!\n');

    // Screenshot 1: Summary/Overview
    console.log('📸 Taking Screenshot 1: Diff Overview (Summary)...');

    // Try to click Summary button
    const summaryButtons = await page.$$('button');
    for (const button of summaryButtons) {
      const text = await page.evaluate(el => el.textContent, button);
      if (text.includes('Summary') || text.includes('📊')) {
        await button.click();
        break;
      }
    }

    await wait(2000);

    // Scroll down to show the results area
    await page.evaluate(() => {
      window.scrollTo(0, 400);
    });
    await wait(500);

    const summaryScreenshot = path.join(SCREENSHOTS_DIR, '01-diff-overview.png');
    await page.screenshot({
      path: summaryScreenshot,
      fullPage: false,
      clip: {
        x: 0,
        y: 300,
        width: 1920,
        height: 1080
      }
    });
    console.log(`   ✓ Saved: ${summaryScreenshot}\n`);

    // Screenshot 2: Function Diff View
    console.log('📸 Taking Screenshot 2: Function Diff View...');

    // Try to click Diff Viewer button
    const diffButtons = await page.$$('button');
    for (const button of diffButtons) {
      const text = await page.evaluate(el => el.textContent, button);
      if (text.includes('Diff Viewer') || text.includes('🔍')) {
        await button.click();
        break;
      }
    }

    await wait(2000);

    // Scroll to top first
    await page.evaluate(() => {
      window.scrollTo(0, 0);
    });
    await wait(500);

    // Try to click on a function if available
    try {
      const functionItem = await page.$('.function-item, [data-function], button[data-testid*="function"]');
      if (functionItem) {
        await functionItem.click();
        await wait(1000);
      }
    } catch (e) {
      console.log('   Note: Could not select a specific function, showing default view');
    }

    // Scroll down to show the diff content
    await page.evaluate(() => {
      window.scrollTo(0, 400);
    });
    await wait(500);

    const diffScreenshot = path.join(SCREENSHOTS_DIR, '02-function-diff.png');
    await page.screenshot({
      path: diffScreenshot,
      fullPage: false,
      clip: {
        x: 0,
        y: 300,
        width: 1920,
        height: 1080
      }
    });
    console.log(`   ✓ Saved: ${diffScreenshot}\n`);

    // Screenshot 3: Graph View
    console.log('📸 Taking Screenshot 3: Graph View...');

    // Scroll to top first
    await page.evaluate(() => {
      window.scrollTo(0, 0);
    });
    await wait(500);

    // Try to click Graph button
    const graphButtons = await page.$$('button');
    for (const button of graphButtons) {
      const text = await page.evaluate(el => el.textContent, button);
      if (text.includes('D3 Graph') || text.includes('🕸️') || text.includes('Interactive') || text.includes('🔗')) {
        await button.click();
        break;
      }
    }

    // Wait for graph to render
    console.log('   Waiting for graph to render...');
    await wait(8000);

    // Scroll down to center the graph
    await page.evaluate(() => {
      window.scrollTo(0, 400);
    });
    await wait(500);

    const graphScreenshot = path.join(SCREENSHOTS_DIR, '03-graph-view.png');
    await page.screenshot({
      path: graphScreenshot,
      fullPage: false,
      clip: {
        x: 0,
        y: 300,
        width: 1920,
        height: 1080
      }
    });
    console.log(`   ✓ Saved: ${graphScreenshot}\n`);

    // Screenshot 4: Detailed Function Diff with Modified Filter
    console.log('📸 Taking Screenshot 4: Modified Function Diff Detail...');

    // Go to Diff Viewer view
    await page.evaluate(() => {
      window.scrollTo(0, 0);
    });
    await wait(500);

    const diffViewerButtons = await page.$$('button');
    for (const button of diffViewerButtons) {
      const text = await page.evaluate(el => el.textContent, button);
      if (text.includes('Diff Viewer') || text.includes('🔍')) {
        await button.click();
        console.log('   ✓ Clicked Diff Viewer tab');
        break;
      }
    }
    await wait(3000); // Wait for view to load

    // Debug: Log what's on the page
    const pageInfo = await page.evaluate(() => {
      const buttons = Array.from(document.querySelectorAll('button'));
      const buttonTexts = buttons.map(b => b.textContent.trim()).filter(t => t.length > 0 && t.length < 100);
      const selects = Array.from(document.querySelectorAll('select'));
      const selectOptions = selects.map(s => ({
        id: s.id,
        name: s.name,
        options: Array.from(s.options).map(o => o.textContent.trim())
      }));

      // Look for function items
      const functionDivs = Array.from(document.querySelectorAll('div.p-4.border.rounded-lg'));
      const functionTexts = functionDivs.map(d => d.textContent.substring(0, 100).trim());

      return {
        buttonCount: buttons.length,
        buttonTexts: buttonTexts.slice(0, 30),
        selectCount: selects.length,
        selectOptions: selectOptions,
        functionDivCount: functionDivs.length,
        functionTexts: functionTexts.slice(0, 5)
      };
    });
    console.log('   Page info:', JSON.stringify(pageInfo, null, 2));

    // Look for filter/dropdown to select "modified" functions
    console.log('   Looking for modified filter...');

    // Try to find and click a select dropdown
    const filterApplied = await page.evaluate(() => {
      const selects = Array.from(document.querySelectorAll('select'));
      for (const select of selects) {
        const options = Array.from(select.options);
        const modifiedOption = options.find(o => o.textContent.toLowerCase().includes('modified'));
        if (modifiedOption) {
          select.value = modifiedOption.value;
          // Trigger change event
          const event = new Event('change', { bubbles: true });
          select.dispatchEvent(event);
          return true;
        }
      }
      return false;
    });

    if (filterApplied) {
      console.log('   ✓ Applied "modified" filter');
      await wait(2000); // Wait for filter to apply
    } else {
      console.log('   ⚠️  Could not find modified filter, showing all functions');
    }

    // Scroll down to see the function list
    await page.evaluate(() => {
      window.scrollTo(0, 400);
    });
    await wait(1000);

    // Find and click on a function item (the divs with class "p-4 border rounded-lg")
    console.log('   Looking for a function to click...');

    const functionClicked = await page.evaluate(() => {
      // Look for function items - they are divs with specific classes
      const functionDivs = Array.from(document.querySelectorAll('div.p-4.border.rounded-lg.hover\\:bg-gray-50.cursor-pointer'));

      if (functionDivs.length > 0) {
        console.log(`Found ${functionDivs.length} function divs`);
        // Click the first one
        functionDivs[0].click();
        return {
          success: true,
          count: functionDivs.length,
          text: functionDivs[0].textContent.substring(0, 100)
        };
      }

      // Fallback: look for any clickable div with cursor-pointer
      const clickableDivs = Array.from(document.querySelectorAll('div.cursor-pointer'));
      for (const div of clickableDivs) {
        const text = div.textContent.trim();
        // Look for function-like content (has function name patterns)
        if (text.match(/[a-zA-Z_][a-zA-Z0-9_]*/) && text.includes('modified')) {
          div.click();
          return {
            success: true,
            count: clickableDivs.length,
            text: text.substring(0, 100),
            fallback: true
          };
        }
      }

      return { success: false, count: 0 };
    });

    if (functionClicked.success) {
      console.log(`   ✓ Clicked function (${functionClicked.count} available)`);
      console.log(`   Function text: ${functionClicked.text}`);
      await wait(2000);
    } else {
      console.log('   ⚠️  Could not find function items to click');
    }

    // Wait for modal/diff to load
    console.log('   Waiting for diff modal/content to load...');

    // Poll for modal/diff content to appear
    let modalLoaded = false;
    let modalAttempts = 0;
    const maxModalAttempts = 30; // 15 seconds max

    while (!modalLoaded && modalAttempts < maxModalAttempts) {
      await wait(500);
      modalAttempts++;

      // Check if modal or diff content is visible
      const modalInfo = await page.evaluate(() => {
        // Look for modal indicators
        const modals = document.querySelectorAll('[role="dialog"], .modal, [class*="modal"], [class*="Modal"], [class*="Dialog"]');
        const hasModal = modals.length > 0;

        // Look for diff content indicators
        const diffContent = document.body.textContent;
        const hasDiffKeywords =
          (diffContent.includes('Source') && diffContent.includes('Target')) ||
          (diffContent.includes('Old') && diffContent.includes('New')) ||
          diffContent.includes('---') || diffContent.includes('+++') ||
          diffContent.includes('Unified') || diffContent.includes('Side-by-Side');

        // Look for code blocks or syntax highlighted content
        const codeBlocks = document.querySelectorAll('pre, code, .monaco-editor, [class*="diff"], [class*="code"]');
        const hasCodeBlocks = codeBlocks.length > 2;

        return {
          hasModal,
          hasDiffKeywords,
          hasCodeBlocks,
          modalCount: modals.length,
          codeBlockCount: codeBlocks.length
        };
      });

      if (modalInfo.hasModal || modalInfo.hasDiffKeywords || modalInfo.hasCodeBlocks) {
        modalLoaded = true;
        console.log(`   ✓ Diff content loaded after ${modalAttempts * 0.5} seconds`);
        console.log(`   Modal info:`, JSON.stringify(modalInfo));
      } else if (modalAttempts % 4 === 0) {
        console.log(`   ... still waiting (${modalAttempts * 0.5}s) - ${JSON.stringify(modalInfo)}`);
      }
    }

    if (!modalLoaded) {
      console.log('   ⚠️  Diff content may not have loaded, proceeding anyway...');
    }

    // Extra wait for content to fully render
    await wait(5000);

    // Debug: Check what's visible
    const visibleContent = await page.evaluate(() => {
      return {
        bodyText: document.body.textContent.substring(0, 500),
        hasMonaco: !!document.querySelector('.monaco-editor'),
        hasPre: document.querySelectorAll('pre').length,
        hasCode: document.querySelectorAll('code').length,
        modals: document.querySelectorAll('[role="dialog"]').length
      };
    });
    console.log('   Visible content:', JSON.stringify(visibleContent, null, 2));

    // Scroll to show the diff content (try different scroll positions)
    await page.evaluate(() => {
      window.scrollTo(0, 300);
    });
    await wait(500);

    const detailedDiffScreenshot = path.join(SCREENSHOTS_DIR, '04-modified-function-diff.png');
    await page.screenshot({
      path: detailedDiffScreenshot,
      fullPage: true
    });
    console.log(`   ✓ Saved: ${detailedDiffScreenshot}\n`);

    console.log('✨ All screenshots captured successfully!\n');
    console.log('Screenshots saved to:');
    console.log(`  - ${summaryScreenshot}`);
    console.log(`  - ${diffScreenshot}`);
    console.log(`  - ${graphScreenshot}`);
    console.log(`  - ${detailedDiffScreenshot}\n`);

  } catch (error) {
    console.error('❌ Error taking screenshots:', error);
    throw error;
  } finally {
    await browser.close();
  }
}

// Run the script
takeScreenshots()
  .then(() => {
    console.log('✅ Screenshot capture complete!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('❌ Failed to capture screenshots:', error);
    process.exit(1);
  });

