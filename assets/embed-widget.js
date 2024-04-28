async function sendGetRequestWithPayload(){let e=await getWidgetConfig(),t=e.publicKey;delete e.publicKey;let r=new URLSearchParams({...e,...await fillMissingConfig(),location:JSON.stringify(await getLocation())});try{let n=await fetch(`https://api.elerem.com/api/v1/embed-recommendations/?${r}`,{method:"GET",headers:{"Content-Type":"application/json",Authorization:`Bearer ${t}`}});await handleResponse(n,e)}catch(o){console.error("Request error:",o)}}async function getWidgetConfig(){var e=JSON.parse(document.currentScript.innerHTML.trim());if(!e.entity||!e.publicKey)throw Error('Widget configuration requires both "entity" and "publicKey" parameters');return e.title=e.title||"We think that this may interest you",e.orientation=e.orientation||"vertical",e.showImage=e.showImage||!0,e.showResume=e.showResume||!0,e.numberRecommendations=e.numberRecommendations||5,e.isTransparent=e.isTransparent||!1,e.locale=e.locale||"en",e.colorTheme=e.colorTheme||"light",e}async function fillMissingConfig(){return{locationHref:window.location.href,baseUri:document.baseURI,docUrl:document.URL,userAgent:navigator.userAgent,language:navigator.language,languages:navigator.languages,screenWidth:window.screen.width,screenHeight:window.screen.height,referrer:document.referrer,documentTitle:document.title,host:window.location.host}}async function getLocation(){let e={enableHighAccuracy:!0,timeout:5e3,maximumAge:0};function t(e){return e.coords}function r(e){return{error:e.message,errorCode:e.code}}return{currentPosition:navigator.geolocation.getCurrentPosition(t,r,e),currentWatch:navigator.geolocation.watchPosition(t,r,e)}}async function handleResponse(e,t){let r=await e.json();e.ok?await populateResults(r.data,t):console.error("Error inside the response:",r.message)}async function populateResults(e,t){let r=document.querySelector(".elerem-widget-container__widget");r&&(await addWidgetStyles(),r.innerHTML=await generateRecommendationHTML(e,t))}async function generateRecommendationHTML(e,t){let r="";r+=`<h2 class="elerem-recommendation-title">${t.title}</h2>`;for(let n=0;n<e.length;n++){let o=e[n],i=o.image?"elerem-recommendation-card--with-image":"elerem-recommendation-card";r+=`
        <a href="${o.url}" class="elerem-recommendation-link">
        <div class="${i}">
          ${o.image?`<img src="${o.image}" alt="Product Image" class="elerem-product-image">`:""}
          <div class="elerem-recommendation-content">
            <span class="elerem-product-title">${o.title}</span>
            ${o.resume?`<span class="elerem-product-resume">${o.resume}</span>`:""}
          </div>
        </div>
      </a>
      `}return r+`<div class="elerem-widget-copyright">
      <a href="https://www.elerem.com/" rel="noopener nofollow" target="_blank">
        <span class="blue-text">Better recommendations from Elerem</span>
      </a>
    </div>`}async function addWidgetStyles(){let e=`
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
    `,t=document.createElement("style");t.innerHTML=e,document.head.appendChild(t)}sendGetRequestWithPayload();
