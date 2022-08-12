let ws = new WebSocket(`ws://${window.location.hostname}:7878`);

ws.onopen = () => {
    console.log("Connection open");
    ws.send(JSON.stringify({ name: "Identify", dat: { username: username, identity: identity } }));
};

ws.onmessage = (evt) => {
    let dat = JSON.parse(evt.data);
    console.log("Received Message:", dat);
};

ws.onclose = () => {
    console.log("Connection closed");
    window.location.href = "/game";
};   