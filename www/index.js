import $ from "jquery";

import * as wasm from "wasm-playground";


wasm.init();

$("#btn").click(function(event) {
    var res = wasm.GeneratedAddress.generate();
    $("#private_key").val(res.private_key());
    $("#address").val(res.address());
});
