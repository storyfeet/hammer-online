
qfetch_mod = {};

qfetch_mod.jsonPost = function(url,data,callback){
    let jsdt = JSON.stringify(data);
    console.log(jsdt);
    fetch(url,{
        method:"POST",
        credentials: "same-origin",
        headers:{
            "Content-Type":"application/json"
        },
        body:jsdt
    }).then(response => response.json())
        .catch(e=>console.log("POST - ERR:"+ e))
        .then(callback);
}

qfetch_mod.jsonGet = function(url,callback){
    fetch(url,{
        method:"GET", 
    }).then(response => response.json())
        .catch(e=>console.log("GET - ERR:" + e))
        .then(callback);
}
