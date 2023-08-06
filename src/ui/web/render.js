var data;


var sketch = function(p) {

    
    p.setup = () => {
        var cnv = p.createCanvas($("#cr-default-canvas").innerWidth(), $("#cr-default-canvas").innerHeight());
        cnv.parent('#cr-default-canvas');
    };
    
    p.draw = () => {
        p.background(255);
        p.fill(155, 155, 155)
        p.rect(10, 10, 10, 10)
    };

    p.windowResized = () => {
        // p.resizeCanvas($("#cr-default-canvas").innerWidth(), $("#cr-default-canvas").innerHeight())
        p.draw()
    } 
};


$(document).ready(() => {
    let myp5 = new p5(sketch);
    console.log(myp5)
})