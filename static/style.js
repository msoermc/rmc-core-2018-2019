//function post() {
//    let command = $("#command-input").val();
//    let url = $(location).attr('href');
//    let send = url + command;
//
//    console.log("Post " + send);
//
//    $.post(send);
//}
$(document).ready(function() {
    // DOCUMENT READY
});

function post_drive() {
    let url = "/drive/1.0/1.0";
    console.log(url);
    fetch(url, {method: "POST"}) // Call fetch function, passing the URL of the server as a parameter
    .then(function (value) {
        alert("Succeeded: " + value);
    })
    .catch(function (reason) {
        alert("Failed: " + value);
    });
}