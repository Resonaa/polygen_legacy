$(async () => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");

    $(".ui.sticky").sticky({ context: ".four" });

    const pid = Number(window.location.href.match(/\d+$/)[0]);
    const postTemplate = juicer.compile($("#post-template").html());
    const commentTemplate = juicer.compile($("#comment-template").html());

    const dat = ajaxSync("get", `/api/post?pid=${pid}`, undefined).msg;
    $(".ui.large.feed").html(postTemplate.render({
        realTime: dat.time,
        deltaTime: deltaTime(dat.time),
        commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: dat.pid }).msg,
        author: dat.author,
        content: await renderText(dat.content),
        home: false,
        viewCount: 0,
        likes: 0
    }));
    $("#author").html(userLink(dat.author));
    $("#time").html(dat.time);
    document.title = `${dat.author}的说说 - polygen`;

    $(".labeled.primary.button").click(() => {
        disableButton(".labeled.primary.button");
        const content = vditor.getValue();

        renderText(content).then(html => {
            if (!$("<div/>").html(html).text().trim()) {
                toast("error", "发送失败", "内容不能为空");
                enableButton(".labeled.primary.button");
            }
            else {
                ajax("post", "/api/comment", { content: content, pid: pid }, msg => {
                    toast("error", "发送失败", msg);
                    enableButton(".labeled.primary.button");
                }, () => {
                    vditor.setValue("");
                    window.location.reload();
                });
            }
        });
    });

    let page = 1;
    let noMore = false;

    function addComment() {
        if (noMore) {
            return;
        }

        ajax("get", `/api/comment?`, { pid: pid, page: page }, undefined, async (dat) => {
            for (let i of dat) {
                $(".comments").append(commentTemplate.render({
                    realTime: i.time,
                    deltaTime: deltaTime(i.time),
                    content: await renderText(i.content),
                    author: i.author,
                }));

                $(".reply").last().click(() => {
                    vditor.setValue(`@${i.author} ${vditor.getValue()}`);
                    vditor.focus()
                });
            }

            if (dat.length < 10) {
                noMore = true;
            }

            Vditor.mathRender($(".comments")[0]);
            Vditor.highlightRender($(".comments")[0]);
        });
    }

    addComment();

    $(window).scroll(() => {
        const scrollTop = $(this).scrollTop();
        const scrollHeight = $(document).height();
        const windowHeight = $(this).height();

        if (scrollHeight - scrollTop - windowHeight <= 10) {
            page++;
            addComment();
        }
    });
});