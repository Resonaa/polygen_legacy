$(async () => {
    $(".menu .item").tab();
    $(".selection.dropdown").dropdown({
        direction: "upward"
    });

    const chatTemplate = juicer.compile($("#chat-template").html());

    async function addChat(event) {
        let comments = $(".comments")[0];
        let isBottom = comments.scrollHeight - comments.clientHeight - comments.scrollTop <= 40;

        $(".comments").append(chatTemplate.render({
            time: new Date().toTimeString().substr(0, 8),
            type: event.name,
            sender: event.dat.sender,
            message: await renderText(event.dat.message),
        }));

        Vditor.mathRender($(".chat-content").last()[0]);
        Vditor.highlightRender($(".chat-content").last()[0]);

        if (isBottom) {
            $(".comments").scrollTop(9999999);
        }

        $(".reply").last().click(() => {
            $("textarea").val(`@${event.dat.sender} ${$("textarea").val()}`).focus();
            $(".selection.dropdown").dropdown("set selected", event.name);
        });
    }

    const ws = new WebSocket(`ws://${window.location.hostname}:7878`);

    ws.onopen = () => {
        console.log("Connection open");
        ws.send(JSON.stringify({ name: "Identify", dat: { username: username, identity: identity } }));
    };

    ws.onmessage = evt => {
        let event = JSON.parse(evt.data);
        console.log("Received Message:", event);

        switch (event.name) {
            case "WorldMessage": case "RoomMessage": {
                addChat(event);
                break;
            }
        }
    };

    ws.onclose = () => {
        console.log("Connection closed");
        window.location.href = "/game";
    };

    window.game = new CatchTheCatGame({
        w: 17,
        h: 17,
        r: 15,
        parent: "catch-the-cat",
        statusBarAlign: "center",
        credit: "polygen",
    });

    function emit(name, dat) {
        ws.send(JSON.stringify({ name: name, dat: dat }));
    }

    $(".submit.button").click(() => {
        const message = $("textarea").val();

        if (message.length < 1) {
            return;
        }

        emit($("#target").val(), message);

        $("textarea").val("");
    });

    $("body").keydown(e => {
        if (e.ctrlKey && e.which == 13) {
            $(".submit.button").click();
        }
        else if (e.which == 13 && document.activeElement != $("textarea")[0]) {
            setTimeout(() => $("textarea").focus(), 300)
        }
    });
});