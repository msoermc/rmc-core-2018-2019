function post_drive() {
    let left = $("#left-drive").val();
    let right = $("#right-drive").val();
    let url = "/drive/" + left + "/" + right;
    post_command(url);
}

function post_kill() {
    let url = "/kill";
    post_command(url);
}

function post_revive() {
    let url = "/revive";
    post_command(url);
}

function post_enable_drive_train() {
    let url = "/enable/drive_train";
    post_command(url);
}

function post_disable_drive_train() {
    let url = "/disable/drive_train";
    post_command(url);
}

function post_brake() {
    let url = "/brake";
    post_command(url);
}

function post_command(url) {
    fetch(url, {method: "POST"})
        .then(function (value) {
            // I'll be damned if I know what to do here.
        })
        .catch(function (reason) {
            alert("Malformed request!");
        });
}