document.getElementById('upload_default').addEventListener('change', handleFileSelect, false)

function handleFileSelect(evt) {

    console.log(evt.target.files[0].name)
    let files = evt.target.files;
    let f = files[0];

    let reader = new FileReader();

    reader.onload = (function(theFile) {
        return function(e) {
            $('#upload_data_buffer').attr('value', `${evt.target.files[0].name}?,${e.target.result}`);
            document.getElementById('upload_data_buffer').dispatchEvent(new Event('input', { bubbles: true }));
        }

    })(f);
    reader.readAsText(f);
}

// var myFile = $('#upload_default').prop('files');
// console.log(myFile);