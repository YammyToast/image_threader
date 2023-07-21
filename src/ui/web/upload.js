document.getElementById('upload_default').addEventListener('change', handleFileSelect, false)

function handleFileSelect(evt) {

    console.log(evt.target.files[0].name)
    let files = evt.target.files;
    let f = files[0];

    let reader = new FileReader();

    reader.onload = (function(theFile) {
        return function(e) {
            var data_image = document.createElement('img');
            data_image.addEventListener('load', function() {
                // console.log(`Height: ${data_image.height}, Width: ${data_image.width}`)
                $('#upload_data_buffer').attr('value', 
                `${evt.target.files[0].name}
                ?,${data_image.height}
                ?,${data_image.width}
                ?,${e.target.result}`);
                document.getElementById('upload_data_buffer').dispatchEvent(new Event('input', { bubbles: true }));
                

            })
            data_image.src = `${e.target.result}`
        }

    })(f);
    reader.readAsDataURL(f);
}

// var myFile = $('#upload_default').prop('files');
// console.log(myFile);