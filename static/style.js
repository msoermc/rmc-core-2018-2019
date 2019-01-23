function init() {
    set_background()
}

function set_background() {
    $.get("https://api.nasa.gov/planetary/apod?api_key=RL2kgIrn0TyKnmAg9EKDW6Y18fR3rsWb8yk2Oou8", function (data) {
        let url = data['url'];
        console.log(url);
        //$(this).css('background-image','url(' + data['hdurl'] + ');');
        document.body.style.backgroundImage = 'url("' + url + '")';
        $("#mars-max").html()
    });
}