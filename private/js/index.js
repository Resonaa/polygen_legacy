$(() => {
    let pid = 0;
    let s = window.location.href.match(/\d+$/);
    if (s != null) {
        pid = Number(s[0]);
    }

    let postEditor = new MarkdownPalettes("#post-editor");

    $("#post").click(() => {
        let content = postEditor.content || "";

        if (!$("<div/>").html(textRenderer(content)).text().trim()) {
            swal("发送失败", "内容不能为空", "error");
            return;
        }

        ajax("post", "/api/post", { content: content, parent: pid }, msg => swal("发送失败", msg, "error"), () => window.location.reload())
    });

    let page = 1;
    let postTemplate = juicer.compile($("#post-template").html());

    function getAuthor(post) {
        if (post.parent == 0) {
            return userLink(post.author);
        }

        return `${userLink(post.author)} <a href="/post/${post.parent}"><i class="far fa-long-arrow-alt-right"></i> ${userLink(ajaxSync("get", "/api/post", { pid: post.parent }).msg.author)}`;
    }

    if (pid != 0) {
        let dat = ajaxSync("get", `/api/post?pid=${pid}`, undefined).msg;
        $("#main-part").prepend(postTemplate.render({
            realTime: dat.time,
            deltaTime: deltaTime(dat.time),
            commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: dat.pid }).msg,
            author: getAuthor(dat),
            content: textRenderer(dat.content),
        }));
        document.title = `${dat.author}的说说 - polygen`;
    }

    function addPost() {
        if ($(".loader").is(":hidden")) {
            return;
        }

        $(".loader").html(`<div class="spinner-border"></div>`);

        ajax("get", `/api/post?`, { parent: pid, page: page }, undefined, dat => {
            for (let i of dat) {
                $("#load-more").before(postTemplate.render({
                    realTime: i.time,
                    deltaTime: deltaTime(i.time),
                    pid: i.pid,
                    commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: i.pid }).msg,
                    content: textRenderer(i.content),
                    author: getAuthor(i),
                    comment: true,
                }));
            }

            if (dat.length < 10) {
                $(".loader").after(`<span id="no-more">没有更多了</span>`).hide();
            } else {
                $(".loader").html("点击查看更多...");
            }
        });
    }

    addPost();

    $("#load-more").click(() => {
        page++;
        addPost();
    });
});