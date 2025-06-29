const { invoke } = window.__TAURI__.core;

// DOM elements
const flagDisplay = document.getElementById('flag-display');
const countryName = document.getElementById('country-name');
const countryCode = document.getElementById('country-code');
const refreshBtn = document.getElementById('refresh-btn');

// Load IP information
async function loadIpInfo() {
  try {
    // Show loading state
    flagDisplay.textContent = '🔄';
    flagDisplay.classList.add('loading');
    countryName.textContent = 'Detecting...';
    countryCode.textContent = 'Please wait';
    refreshBtn.disabled = true;

    // Invoke Tauri command
    const result = await invoke('get_public_ip_info');
    
    // Update UI with results
    flagDisplay.textContent = result.flag_emoji;
    countryName.textContent = result.name;
    countryCode.textContent = result.code === 'EARTH' ? 'Location Unknown' : `Country Code: ${result.code}`;
    
    // Remove loading state
    flagDisplay.classList.remove('loading');
    refreshBtn.disabled = false;
    
  } catch (error) {
    console.error('Error fetching IP info:', error);
    
    // Show error state
    flagDisplay.textContent = '🌍';
    flagDisplay.classList.remove('loading');
    countryName.textContent = 'Detection Failed';
    countryCode.textContent = 'Please try again';
    refreshBtn.disabled = false;
  }
}

// Refresh button handler
refreshBtn.addEventListener('click', loadIpInfo);

// Load IP info on page load
document.addEventListener('DOMContentLoaded', loadIpInfo);

// Auto-refresh every 5 minutes
setInterval(loadIpInfo, 5 * 60 * 1000);
