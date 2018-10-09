hint_mod = {};

hint_mod.new = function(node,pname,rooter){
    res = {
        toDrop:["Role","Goal","Skill","Trait"],
        selected:[]
    };

    let drops_selected = function(){
        let ds = res.toDrop.slice();
        let sel = res.selected.slice();
        if (sel.length === 0 ) return false;
        for (i in sel){
            let si = sel[i];
            if (!si.isPlayer) return false;
            if (si.src !== pname) return false;
            
            let dex = ds.indexOf(si.card.kind);
            if (dex === -1) {
                return false;
            }
            ds.splice(dex,1);
        }
        return true;
    }


    res.showMessage = function(m){
        node.innerHTML = m;
    }

    res.hint = function(){
        while (node.hasChildNodes()) node.removeChild(node.lastChild);
        //try hint ideas in preference order
        if (this.toDrop.length >0 ){ //drop cards

            if (drops_selected()){
                dbutt = document.createElement("button");
                dbutt.innerHTML = "Drop Cards";
                dbutt.onclick = ()=> {
                    rooter.request(res.selected.map(r=>{return{DropCard:r.card}}));
                }
                node.appendChild(dbutt);
                return;
            }
            
            let mess = "Select one of each Kind to drop: ";
            for (p in this.toDrop){
                mess += this.toDrop[p] + ", ";
            }
            this.showMessage(mess);
            return;

        }//drop

        this.showMessage("What would you like to do?");

    }

    res.selectCard = function(card,src,isPlayer){
        res.selected.push({
            card:card,
            src:src,
            isPlayer:isPlayer,
        });
        res.hint();
    }

    res.deselectCard = function(card,src,isPlayer){
        res.selected = res.selected.filter(item => item.card !== card);
        res.hint();
    }

    res.countDrop = function(kind,plname){
        if (plname == pname) {
            let dex = res.toDrop.indexOf(kind);  
            if (dex != -1) {
                res.toDrop.splice(dex,1);
            }
        }
    }

    return res;
}
