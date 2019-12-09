import {set_text, real_text} from './Cargo.toml'
let {millis} = p5.prototype


const newsAPIKey = 'cadf03152b7a4215896a1082e2025109'

/**
 * Fetch a news article and give it to Rust for initialization
 */

fetch(`https://newsapi.org/v2/top-headlines?country=nl&apiKey=cadf03152b7a4215896a1082e2025109`).then(async response => {
    const {articles} = await response.json()

    let aggregatedArticles = ""
    for (const article of shuffleArray(articles.slice(0, 3))) {
        aggregatedArticles += `${article.title} ------ `
    }

    set_text(aggregatedArticles)

    new p5();
    let $portrait = document.querySelector('#portrait')
    setInterval(() => {
        $portrait.innerHTML = real_text(innerWidth, innerHeight, millis())
    }, 16)
})


/**
 * Utils
 */
function shuffleArray(a) {
    for (let i = a.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [a[i], a[j]] = [a[j], a[i]];
    }
    return a;
}
