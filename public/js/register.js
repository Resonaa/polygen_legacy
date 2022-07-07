$(() => {
    $("#submit").click(() => {
        $(".spinner-border").show();
        $("#submit").attr("disabled", 0);

        function show(s) {
            swal("注册失败", s, "error");
            $(".spinner-border").hide();
            $("#submit").removeAttr("disabled");
        }

        if ($("#password")[0].value != $("#password2")[0].value) {
            show("两次输入的密码不一致");
            return;
        }

        ajax("post", "/register", { "username": $("#username")[0].value, "password": $("#password")[0].value }
            , msg => show(msg), () => window.location.href = '/');
    });
    $("#login").keydown(e => {
        let keyCode = e.which || e.keyCode;
        if (keyCode == 13) {
            $("#submit").click();
        }
    });
})