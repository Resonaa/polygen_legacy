$(() => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");

    function changeCaptcha() {
        $("#captcha").val("");
        $(".captcha")[0].src = "/api/captcha?t=" + Math.random();
    }

    $(".captcha").click(changeCaptcha);

    $(".submit").click(() => {
        disableButton(".submit");

        function show(s) {
            toast("error", "注册失败", s);
            enableButton(".submit");
            changeCaptcha();
        }

        const username = $("#username").val(), password = $("#password").val(), password2 = $("#password2").val(), captcha = $("#captcha").val();

        if (password != password2) {
            show("两次输入的密码不一致");
            return;
        }

        ajax("post", "/register", { username: username, password: password, captcha: captcha }
            , msg => show(msg), () => window.location.href = "/");
    });

    $("form").keydown(e => {
        const keyCode = e.which || e.keyCode;
        if (keyCode == 13) {
            $(".submit").click();
        }
    });
});