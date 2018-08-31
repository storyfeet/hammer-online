let player_mod = {};


player_mod.newPlayer = function(name,par_div){
    let view = document.createElement("div");
    view.classList.add("player_box");

    let namebox = document.createElement("h2");
    namebox.innerHTML = name;
    view.appendChild(namebox);

    let mbox = document.createElement("div");
    mbox.classList.add("message_box");
    view.appendChild(mbox);

    let res = {
        name:name,
        cards:[],
        view:view,
        toDrop:["Role","Goal","Skill","Trait"],
        mbox : mbox,
    };

    let select_card = c => ()=>{ //returns function
        c.selected = ! c.selected ;
        if (c.selected) {
            c.view.classList.add("selected"); 
        }else {
            c.view.classList.remove("selected");
        }
        res.hint();
    }

    res.addCard = function(c) {
        cview = document.createElement("div");
        cview.appendChild(document.createTextNode(c.name));
        cview.classList.add("card_box","kind_" + c.kind);

        let crd = {card:c,view:cview};

        cview.onclick = select_card(crd);
        this.cards.push(crd);
        this.view.appendChild(cview);
    };

    res.getSelected = function(){
        let fres = [];
        for (p in this.cards){
            if (this.cards[p].selected){
                fres.push(this.cards[p]);
            }
        }
        return fres;
    }
    res.showMessage = function(m){
        res.mbox.innerHTML = m;
    }
    res.dropCard = function(c){
    }

    let drops_selected = function(){
        let ds = res.toDrop.slice();
        let sel = res.getSelected();
        if (sel.length === 0 ) return false;
        for (i in sel){
            let dex = ds.indexOf(sel[i].card.kind);
            if (dex === -1) {
                return false;
            }
            ds.splice(dex,1);
        }
        return true;
    }

    res.hint = function(c){
        while (mbox.hasChildNodes()) mbox.removeChild(mbox.lastChild);
        //try hint ideas in preference order
        if (this.toDrop.length >0 ){ //drop cards

            if (drops_selected()){
                dbutt = document.createElement("button");
                dbutt.innerHTML = "Drop Cards";
                dbutt.onclick = ()=> {
                    console.log("Dropping", res.getSelected());
                }
                mbox.appendChild(dbutt);
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

    par_div.appendChild(view);

    return res;
}
