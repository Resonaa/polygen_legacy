$(() => {
    $("#submit").click(() => {
        if ($("#password")[0].value != $("#password2")[0].value) {
            toast("error", "注册失败", "两次输入的密码不一致");
            return;
        }

        $.ajax({
            type: "post",
            url: "/register",
            data: JSON.stringify({ "username": $("#username")[0].value, "password": $("#password")[0].value }),
            dataType: "json",
            success: function (res) {
                if (res.status == "success") {
                    toast("success", "注册成功", "欢迎", 700, () => window.location.href = '/');
                } else {
                    toast("error", "注册失败", res.msg);
                }
            }
        });
    });
    $(document).keydown(e => {
        let keyCode = e.which || e.keyCode;
        if (keyCode == 13) {
            $("#submit").click();
        }
    });
})