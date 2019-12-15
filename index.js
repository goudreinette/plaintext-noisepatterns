import {render, charForPos} from './Cargo.toml'
let {noise, random, dist, millis, mp} = p5.prototype
let r = 3


new p5();

window.render = render
window.$portrait = document.querySelector('#portrait')
// console.log(render(innerWidth, innerHeight, millis()))


setInterval(() => {
    $portrait.innerHTML = render(innerWidth, innerHeight, millis())
}, 16)

