async function sendGetRequestWithPayload() {
  const endpoint = 'http://localhost:8001/api/v1/embed-recommendations/';
  const configWidget = await getWidgetConfig();
  const token = configWidget.publicKey;
  delete configWidget.publicKey;
  const params = new URLSearchParams({
    ...configWidget,
    ...await fillMissingConfig(),
    location: JSON.stringify(await getLocation()),
  });
  try {
    const response = await fetch(`${endpoint}?${params}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
    });
    await handleResponse(response);
  } catch (error) {
    console.error('Request error:', error);
  }
}

async function getWidgetConfig() {
  var scriptContent = document.currentScript.innerHTML.trim();
  var config = JSON.parse(scriptContent);
  if (!config.entity || !config.publicKey) {
    throw new Error('Widget configuration requires both "entity" and "publicKey" parameters');
  }
  config.title = config.title || 'We think that this may interest you';
  config.orientation = config.orientation || 'vertical';
  config.showImage = config.showImage || true;
  config.showResume = config.showResume || true;
  config.numberRecommendations = config.numberRecommendations || 5;
  config.isTransparent = config.isTransparent || false;
  config.locale = config.locale || 'en';
  config.colorTheme = config.colorTheme || 'light';
  return config;
}

async function fillMissingConfig() {
  return {
    locationHref: window.location.href,
    baseUri: document.baseURI,
    docUrl: document.URL,
    userAgent: navigator.userAgent,
    language: navigator.language,
    languages: navigator.languages,
    screenWidth: window.screen.width,
    screenHeight: window.screen.height,
    referrer: document.referrer,
    documentTitle: document.title,
    host: window.location.host,
  }
}

async function getLocation() {
  const options = {
    enableHighAccuracy: true,
    timeout: 5000,
    maximumAge: 0,
  };

  function success(pos) {
    return pos.coords;
  }

  function error(err) {
    return { error: err.message, errorCode: err.code };
  }

  return {
    currentPosition: navigator.geolocation.getCurrentPosition(success, error, options),
    currentWatch: navigator.geolocation.watchPosition(success, error, options)
  };
}

async function handleResponse(response) {
  const data = await response.json();
  if (response.ok) {
    await populateResults(data.data);
  } else {
    console.error('Error inside the response:', data.message);
  };
}

async function populateResults(data) {
  const widgetContainer = document.querySelector('.elerem-widget-container__widget');
  if (widgetContainer) {
    await addWidgetStyles();
    widgetContainer.innerHTML = await generateRecommendationHTML(data);
  };
}

async function generateRecommendationHTML(data) {
  let html = '';

  html += `<h2 class="elerem-recommendation-title">We think that this may interest you</h2>`;

  for (let i = 0; i < data.length; i++) {
    const item = data[i];

    const cardClass = item.image ? 'elerem-recommendation-card--with-image' : 'elerem-recommendation-card';

    html += `
        <a href="${item.url}" class="elerem-recommendation-link">
        <div class="${cardClass}">
          ${item.image ? `<img src="${item.image}" alt="Product Image" class="elerem-product-image">` : ''}
          <div class="elerem-recommendation-content">
            <span class="elerem-product-title">${item.title}</span>
            ${item.resume ? `<span class="elerem-product-resume">${item.resume}</span>` : ''}
          </div>
        </div>
      </a>
      `;
  }

  html += `<div class="elerem-widget-copyright">
      <a href="https://www.elerem.com/" rel="noopener nofollow" target="_blank">
        <span class="blue-text">Better recommendations from Elerem</span>
      </a>
    </div>`;

  return html;
}

async function addWidgetStyles() {
  const styles = `
    .elerem-widget-container__widget {
        background-color: #fff;
        border-radius: 5px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        padding: 10px;
      }
  
      .elerem-recommendation-card {
        display: flex;
        align-items: center;
        padding: 10px;
        border-radius: 5px;
        border: 1px solid #ccc;
        margin-bottom: 10px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      }
  
      .elerem-recommendation-card--with-image {
        display: flex;
        align-items: center;
        padding: 10px;
        border-radius: 5px;
        border: 1px solid #ccc;
        margin-bottom: 10px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      }

      .elerem-product-resume {
        display: block;
        margin-top: 5px;
        font-size: 14px;
        color: #999;
      }      
  
      .elerem-product-image {
        width: 80px;
        height: 80px;
        object-fit: cover;
        margin-right: 10px;
      }
  
      .elerem-recommendation-content {
        flex: 1;
      }
  
      .elerem-product-link {
        color: #333;
        text-decoration: none;
        font-weight: bold;
      }
  
      .elerem-widget-copyright a {
        font-size: 12px;
        color: #888;
        text-decoration: none;
      }
      
      .elerem-widget-copyright a:hover {
        text-decoration: underline;
      }
      
      .elerem-widget-copyright span {
        font-size: 12px;
      }
    `;

  const styleElement = document.createElement("style");
  styleElement.innerHTML = styles;
  document.head.appendChild(styleElement);
}

sendGetRequestWithPayload();