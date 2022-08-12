$(() => {
    let roomTemplate = juicer.compile($("#room-template").html());

    let translation = {
        Quadrilateral: "四边形", Hexagon: "六边形", Random: "随机地图",
    };

    function addRoom(room) {
        let players = "", playerCnt = 0;

        for (let player in room.players) {
            players += userLink(player) + ", ";
            playerCnt++;
        }

        players = `${playerCnt}玩家: ` + players;

        $("#ongoing-games").append(roomTemplate.render({
            status: room.status.toLowerCase(),
            mode: translation[room.map.config.mode],
            map: translation[room.map.config.tp],
            players: players.substring(0, players.length - 2),
            rid: room.rid
        }));
    }

    ajax("get", "/api/room", undefined, undefined, rooms => {
        $("#ongoing-games").html("");

        for (let room of rooms) {
            addRoom(room);
        }

        $("[data-bs-toggle='tooltip']").each((_, e) => new bootstrap.Tooltip(e));
    });
});