#!/usr/bin/env node

const puppeteer = require('puppeteer');
const path = require('path');
const fs = require('fs');
const readline = require('readline');

const wait = (ms) => new Promise(resolve => setTimeout(resolve, ms));

const SCREENSHOTS_DIR = path.join(__dirname, 'screenshots');
const BASE_URL = 'http://localhost:3000';

// Ensure screenshots directory exists
if (!fs.existsSync(SCREENSHOTS_DIR)) {
  fs.mkdirSync(SCREENSHOTS_DIR, { recursive: true });
}

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function askQuestion(query) {
  return new Promise(resolve => rl.question(query, resolve));
}

async function takeManualScreenshot() {
  console.log('🚀 Manual Screenshot Tool\n');
  console.log('This tool will open a browser and let you manually navigate to the view you want to screenshot.\n');
  
  const browser = await puppeteer.launch({
    headless: false,
    defaultViewport: {
      width: 1920,
      height: 1080
    },
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--window-size=1920,1080',
      '--start-maximized'
    ]
  });

  try {
    const page = await browser.newPage();
    
    console.log('📂 Opening Smart Diff...');
    await page.goto(BASE_URL, { waitUntil: 'networkidle2', timeout: 30000 });
    
    // Fix CSS overflow issues
    await page.evaluate(() => {
      const style = document.createElement('style');
      style.textContent = `
        body, html {
          overflow-x: hidden !important;
        }
      `;
      document.head.appendChild(style);
    });
    
    await wait(2000);
    
    console.log('\n✅ Browser is open!');
    console.log('\n📋 Instructions:');
    console.log('1. Fill in the source and target directories');
    console.log('2. Click "Start Comparison" and wait for it to complete');
    console.log('3. Click on the "🔍 Diff Viewer" tab');
    console.log('4. Use the filter dropdown to select "modified"');
    console.log('5. Click on a modified function to open the diff modal');
    console.log('6. Wait for the modal to fully load');
    console.log('7. Come back here and press Enter when ready\n');
    
    await askQuestion('Press Enter when you are ready to take the screenshot...');
    
    console.log('\n📸 Taking screenshot...');
    
    // Take full page screenshot
    const screenshot = path.join(SCREENSHOTS_DIR, '04-modified-function-diff.png');
    await page.screenshot({
      path: screenshot,
      fullPage: true
    });
    
    console.log(`✓ Screenshot saved: ${screenshot}\n`);
    
    const another = await askQuestion('Take another screenshot? (y/n): ');
    
    if (another.toLowerCase() === 'y') {
      const filename = await askQuestion('Enter filename (without .png): ');
      const screenshotPath = path.join(SCREENSHOTS_DIR, `${filename}.png`);
      await page.screenshot({
        path: screenshotPath,
        fullPage: true
      });
      console.log(`✓ Screenshot saved: ${screenshotPath}\n`);
    }
    
  } catch (error) {
    console.error('❌ Error:', error);
  } finally {
    rl.close();
    await browser.close();
    console.log('\n✅ Done!');
  }
}

takeManualScreenshot();

