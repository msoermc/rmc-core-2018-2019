$(document).ready(function () {
    // DOCUMENT READY
});

function post_drive() {
    let url = "/drive/1.0/1.0";
    console.log(url);
    fetch(url, {method: "POST"})
        .then(function (value) {
            // I'll be damned if I know what to do here.
        })
        .catch(function (reason) {
            // I'll be damned if I know what to do here.
        });
}