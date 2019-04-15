setInterval(get_state, 500);

function drive_from_form() {
    let left = parseFloat($("#left-drive").val());
    let right = parseFloat($("#right-drive").val());
    drive(left, right)
}

function get_state() {
    fetch("/robot",
        {
            method: "GET",
            headers: {
                Accept: 'application/json',
            },
        },
    )
        .then(response => {
            // TODO find a better way to display state.
            response.json().then(value => {
                $("#state-view").text(JSON.stringify(value));
            })

        })
        .catch(function (reason) {
            alert(reason);
        });
}

function kill() {
    putRobot({life: "Dead"});
}

function revive() {
    putRobot({life: "Alive"});
}

function switch_to_drive() {
    putRobot({mode: "Driving"});
}

function switch_to_dump() {
    putRobot({mode: "Dumping"});
}

function switch_to_dig() {
    putRobot({mode: "Digging"});
}

function brake() {
    putData("/robot/drive", "Brake")
}

function dig() {
    putIntake({digger: "Dig"})
}

function stop_digging() {
    putIntake({digger: "Stop"})
}

function dump() {
    putDumper("Dig")
}

function reset_dumper() {
    putDumper("Reset")
}

function stop_dumping() {
    putDumper("Stop")
}

function raise_actuators() {
    putIntake({actuator: "Raise"});
}

function lower_actuators() {
    putIntake({actuator: "Lower"});
}

function stop_actuators() {
    putIntake({actuator: "Stop"});
}

function putIntake(action) {
    let url = "/robot/intake";
    putData(url, action);
}

function putDumper(action) {
    putData("/robot/dumper", action)
}

function drive(left, right) {
    let url = "/robot/drive/";
    let data = {
        Drive: {
            left: left,
            right: right,
        }
    };

    console.log("Drive: ", data);

    putData(url, data)
}

function putRobot(action) {
    putData("/robot", action)
}

function putData(url, data) {
    return fetch(url, {
        method: "PUT",
        cache: "no-cache",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data)
    })
        .then(response => {
            console.log("Fetch succeeded: ", response)
        })
        .catch(error => {
            return console.log("Fetch failed: ", JSON.stringify(error));
        });
}