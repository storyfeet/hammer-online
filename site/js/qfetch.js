
qfetch_mod = {};

qfetch_mod.jsonPost = function(url,data,callback){
    let jsdt = JSON.stringify(data);
    console.log(jsdt);
    fetch(url,{
        method:"POST",
        headers:{
            "Content-Type":"application/json"
        },
        body:jsdt
    }).then(response => response.json())
        .catch(e=>console.log("load Error:"+ e))
        .then(callback);
}
