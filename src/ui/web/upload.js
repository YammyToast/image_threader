document.getElementById('upload_default').addEventListener('change', handleFileSelect, false)

function handleFileSelect(evt) {
    let files = evt.target.files;
    let f = files[0];

    let reader = new FileReader();

    reader.onload = (function(theFile) {
        return function(e) {
            console.log(e.target.result);
        }

    })(f);
    reader.readAsText(f);
}

// var myFile = $('#upload_default').prop('files');
// console.log(myFile);