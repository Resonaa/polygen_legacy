$(() => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");
    $(".selection.dropdown").dropdown();

    const roomTemplate = juicer.compile($("#room-template").html());

    const translation = {
        Quadrilateral: "四边形", Hexagon: "六边形", Random: "随机地图",
    };

    function addRoom(room) {
        let players = "", playerCnt = 0;

        for (let player in room.players) {
            players += userLink(player) + ", ";
            playerCnt++;
        }

        players = `${playerCnt}玩家: ` + players;

        $("tbody").append(roomTemplate.render({
            ongoing: room.ongoing,
            mode: translation[room.mode],
            map: translation[room.type],
            players: players.substring(0, players.length - 2),
            rid: room.rid
        }));
    }

    ajax("get", "/api/room", undefined, undefined, rooms => {
        for (let room of rooms) {
            addRoom(room);
        }
    });
});