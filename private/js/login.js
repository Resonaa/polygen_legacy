$(() => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");

    function changeCaptcha() {
        $("#captcha").val("");
        $(".captcha")[0].src = "/api/captcha?t=" + Math.random();
    }

    $(".captcha").click(changeCaptcha);

    $(".submit").click(() => {
        disableButton(".submit");

        ajax("post", "/login", { username: $("#username").val(), password: $("#password").val(), captcha: $("#captcha").val() }, msg => {
            $("body").toast({
                class: "error",
                title: "登陆失败",
                message: msg
            });

            enableButton(".submit");
            changeCaptcha();
        }, () => window.location.href = "/");
    });

    $("form").keydown(e => {
        let keyCode = e.which || e.keyCode;
        if (keyCode == 13) {
            $(".submit").click();
        }
    });
});