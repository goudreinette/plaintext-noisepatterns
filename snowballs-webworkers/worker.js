importScripts('https://unpkg.com/workway/worker.js')

function random(min, max) { // min and max included
    return Math.random() * (max - min + 1) + min;
}


workway({
    snowballs: [],
    update(innerWidth, innerHeight, charHeight, charWidth) {
        if (Math.random() > 0.9 && this.snowballs.length < 5) {
            this.snowballs.push({
                x: Math.random() * innerWidth / charWidth * 2,
                y: -5,
                vx: Math.random() * -1,
                vy: Math.random() * 2,
                size: Math.random() * 3 + 2,
            })
        }

        for (let i = 0; i < this.snowballs.length; i++) {
            const snowball = this.snowballs[i]
            snowball.x += snowball.vx
            snowball.y += snowball.vy
            snowball.size +=  Math.random() / 4
            if (snowball.y > innerHeight / charHeight + snowball.size || snowball.x < -snowball.size) {
                this.snowballs.splice(i, 1)
            }
        }

        return this.snowballs
    }
})
