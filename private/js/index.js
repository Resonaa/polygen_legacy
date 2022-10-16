$(() => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");

    ajax("get", "/api/announcement", undefined, () => $("#announcement").html("数据库错误"), async (dat) => {
        const announcementTemplate = juicer.compile($("#announcement-template").html());

        for (let i of dat) {
            $("#announcement").append(announcementTemplate.render({
                aid: i.aid,
                title: i.title,
                content: await renderText(i.content)
            }));
        }
    });

    $(".labeled.primary.button").click(() => {
        disableButton(".labeled.primary.button");
        const content = vditor.getValue();

        renderText(content).then(html => {
            if (!$("<div/>").html(html).text().trim()) {
                toast("error", "发送失败", "内容不能为空");
                enableButton(".labeled.primary.button");
            }
            else {
                ajax("post", "/api/post", content, msg => {
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
    const postTemplate = juicer.compile($("#post-template").html());
    let noMore = false;

    function addPost() {
        if (noMore) {
            return;
        }

        ajax("get", `/api/post?`, { page: page }, undefined, async (dat) => {
            for (let i of dat) {
                $(".feed").append(postTemplate.render({
                    realTime: i.time,
                    deltaTime: deltaTime(i.time),
                    pid: i.pid,
                    commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: i.pid }).msg,
                    content: await renderText(i.content),
                    author: i.author,
                    home: true,
                    viewCount: 0,
                    likes: 0
                }));
            }

            if (dat.length < 10) {
                noMore = true;
            }

            Vditor.mathRender($(".feed")[0]);
            Vditor.highlightRender($(".feed")[0]);
        });
    }

    addPost();

    $(window).scroll(() => {
        const scrollTop = $(this).scrollTop();
        const scrollHeight = $(document).height();
        const windowHeight = $(this).height();

        if (scrollHeight - scrollTop - windowHeight <= 10) {
            page++;
            addPost();
        }
    });
});