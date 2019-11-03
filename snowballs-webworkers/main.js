
new p5();

const charWidth = 8.4
const charHeight = 9.0
let t = 0
let lineLength = 0


workway('worker.js').then(async ({namespace}) => {
    window.draw = function () {


         actualDraw().then(lines => document.body.innerHTML = lines)


        async function actualDraw() {
            let snowballs = await namespace.update(innerWidth, innerHeight, charHeight, charWidth);

            // The update loop
            let lines = ""
            lineLength = innerWidth / charWidth
            for (let y = 0; y < innerHeight / charHeight; y++) {
                let str = ""
                for (let x = 0; x < innerWidth / charWidth; x++) {
                    str += charForPos(snowballs, x, y, char)
                }
                lines += str + "\n"
            }

            return lines
        }


        /**
         * Draw, basically
         */
        function charForPos(snowballs,x, y, char) {
            if (mouseIsPressed) {
                // speed lines
                if ((y <= 1 ) && x <= e.clientX / 8.4) {
                    return "&nbsp;"
                }
            }

            for (const snowball of snowballs) {
                if (dist(snowball.x, snowball.y, x, y) < snowball.size) {
                    return "&nbsp;"
                }
            }

            return '&block;'
        }
    }
});
