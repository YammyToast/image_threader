var data;


var sketch = function(p) {
    var img;

    p.setup = () => {
        var cnv = p.createCanvas($("#cc-default-canvas").innerWidth(), $("#cc-default-canvas").innerHeight());
        cnv.parent('#cc-default-canvas');
        img = p.loadImage(`${data}`)
    };
    
    p.draw = () => {
        p.background(255);
        p.image(img, 0, 0, $("#cc-default-canvas").innerWidth(), $("#cc-default-canvas").innerHeight())
    };

    p.windowResized = () => {
        p.resizeCanvas($("#cc-default-canvas").innerWidth(), $("#cc-default-canvas").innerHeight())

    } 
};


$(document).ready(() => {
    data = $('#render_data_buffer').val()
    let myp5 = new p5(sketch);
})