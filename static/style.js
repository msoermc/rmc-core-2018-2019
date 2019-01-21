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

function post() {
    var hostname = $(location).attr('host');
    fetch(hostname, {method: "POST"}) // Call fetch function, passing the URL of the server as a parameter
    .then(function (value) {
        // Handle data received from server
    })
    .catch(function (reason) {
        // Catch errors that are thrown
    });
}