static mut CHAR_VEC: Vec<char> = Vec::new();
static mut COUNT:i32=0;
static mut LAST_COUNT:i32=0;
static mut LINE:u32=0;
static mut LAST_LINE:u32=0;
static mut MAX:i32=0;

// tahlil gar loghavi *************************************************************************************************************************888

fn getchar() -> char{
    unsafe{
        let c = CHAR_VEC[LAST_COUNT as usize];
        LAST_COUNT+=1;
        c
    }
}

fn ungetchar(){
    unsafe{
        LAST_COUNT-=1;
    }
}

fn droptoken(){
    unsafe{
        COUNT=LAST_COUNT;
        LINE=LAST_LINE;
    }
}

fn skeep_space(mut c:char) -> char{
    while c=='\n' || c=='\t' || c=='\r' || c==' '{
        unsafe{
            if LAST_COUNT>=MAX{break}
            if c=='\n'{LAST_LINE+=1;}
        }
        c=getchar();
    }
    c
}

fn nexttoken() -> (String,String,i8){
    unsafe{
        LAST_COUNT=COUNT;
        LAST_LINE=LINE;
        if COUNT==MAX {
            return ("the end".to_string(),"end".to_string(),-1)
        }
    }
    let mut c = getchar();
    // skeep \n \t \r space
    c=skeep_space(c);
    // skeep comment
    while c=='#'{
        c=getchar();
        while c!='\n'{
            unsafe{
                if LAST_COUNT>=MAX{break}
            }
            c=getchar();
        }
        unsafe{
            if c=='\n'{LAST_LINE+=1;}
            if LAST_COUNT>=MAX{return ("".to_string(),"".to_string(),1)}
        }
        c=getchar();
    }
    c=skeep_space(c);
    // tokken

    // operator
    if !(c.is_alphabetic() || c.is_numeric()){
        if  c=='%' || c==';' || c=='/' || c==',' || c==')' || c=='(' || 
            c=='}' || c=='{' || c==':' || c=='[' || c==']' || c==','{
            return (c.to_string(),"oprt".to_string(),1)
        }

        else if  c=='*' || c=='+' || c=='-' || c=='!' || c=='<' || c=='>' || c=='=' || c=='|' || c=='&'{
            let mut tokken=String::from("");
            
            tokken.push(c);
            c = getchar();
            
            if  (tokken=="*" && c=='*') || (tokken=="+" && c=='+') || (tokken=="-" && c=='-') ||
                (tokken=="!" && c=='=') || (tokken=="<" && c=='=') || (tokken==">" && c=='=') ||
                (tokken=="=" && c=='=') || (tokken=="|" && c=='|') || (tokken=="&" && c=='&') {
                    tokken.push(c);
            }
            else{
                if tokken=="|" || tokken=="&"{
                    unsafe{return (format!("error in line {}: '{}' does not sopport did you mean '{}{}' ?",LAST_LINE+1,tokken,tokken,tokken),"expr".to_string(),-1)}
                }
                ungetchar();
            }
            return (tokken,"oprt".to_string(),1)
        }
    }
    //numbers
    else if c.is_numeric(){
        let mut num : u32 = c.to_digit(10).unwrap();
        let mut temp_num : u32;
        let mut isu : bool=false;
        let mut pnum:u32=0;
        loop{
            unsafe{if LAST_COUNT>=MAX{break}}
            c = getchar();
            if c=='.'{
                if !isu{
                    isu=true;
                    continue
                }else {
                    unsafe{return (format!("error in line {}: numbers cant be like this n.n.n",LAST_LINE+1),"num".to_string(),-1)}
                }
            }
            if c.is_numeric(){
                temp_num=c.to_digit(10).unwrap();
                num*=10;
                num+=temp_num;
                if isu{
                    pnum+=1;
                }
            }
            else if c.is_alphabetic(){
                unsafe{return (format!("error in line {}: ID does not start with number",LAST_LINE+1),"num".to_string(),-1)}
            }
            else{
                break
            }
        }
        if isu && pnum==0 {unsafe{return (format!("error in line {}: numbers cant be like this n.",LAST_LINE+1),"num".to_string(),-1)}}
        let mut i:u32=0;
        let mut ii:u32=1;
        let ss:f32;
        while i<pnum{
            ii*=10;
            i+=1;
        }
        ss=(num as f32)/(ii as f32);
        ungetchar();
        return (ss.to_string(),"num".to_string(),1)
    }
    // VARs and ... 
    else if c.is_alphabetic(){
        let mut tmpstr =String::from(""); 
        loop{
            if c.is_alphabetic() || c.is_numeric(){
                tmpstr.push(c);
                unsafe{if LAST_COUNT>=MAX{break}}
                c=getchar();
            }else{
                break
            }
        }
        ungetchar();
        
        return (tmpstr.to_string(),"id".to_string(),1)
        
    }
    ("".to_string(),"non".to_string(),1)
}

// tahlil gar nahvi *************************************************************************************************************************

struct VarStruct {
    var_value:String,
    var_name:String,
    var_type:String,
    var_line: u32,
    var_is_assing: bool,
    var_is_used: bool
}
struct BodyStruc{
    header_vars: Vec<VarStruct>,
    vars: Vec<VarStruct>,
    is_func: bool,
    func_name: String,
    func_type: String,
}

impl VarStruct{
    fn construct(var_value:String, var_name:String, var_type:String, var_line: u32, var_is_assing: bool, var_is_used: bool) -> VarStruct {
        VarStruct {
            var_value:var_value,
            var_name:var_name,
            var_type:var_type,
            var_line: var_line,
            var_is_assing: var_is_assing,
            var_is_used: var_is_used
        }
    }
}

impl BodyStruc{
    
    fn construct(is_func: bool, func_name: String,  func_type: String)->BodyStruc{
        BodyStruc {
            header_vars:Vec::new(),
            vars:Vec::new(),
            is_func:is_func,
            func_name:func_name,
            func_type: func_type,
        }
    }
    
    fn assing_var(&mut self, var_name:&str, var_value:&str) -> bool{
        let mut var_exist=false;
        if !self.header_vars.is_empty(){
            let mut header_vars_len = self.header_vars.len()-1;
            loop {
                let header_var = &mut self.header_vars[header_vars_len];
                if var_name==header_var.var_name{
                    //here ---
                    var_exist=true;
                    header_var.var_is_assing=true;
                    header_var.var_value=var_value.to_string();
                }
                if header_vars_len<=0 || var_exist{
                    break;
                }else{
                    header_vars_len-=1;
                }
            }
        }
        if !self.vars.is_empty() && !var_exist{
            let mut body_vars_len=self.vars.len()-1;
            loop {
                let body_var = &mut self.vars[body_vars_len];
                if var_name==body_var.var_name{
                    // here ---
                    var_exist=true;
                    body_var.var_is_assing=true;
                    body_var.var_value=var_value.to_string();
                }
                if body_vars_len<=0 || var_exist{
                    break;
                }else{
                    body_vars_len-=1;
                }
            }
        }

        var_exist
    }

    fn using_var(&mut self, var_name:&str) ->bool{
        let mut var_exist=false;
        if !self.header_vars.is_empty(){
            let mut header_vars_len=self.header_vars.len()-1;
            loop {
                let header_var = &mut self.header_vars[header_vars_len];
                if var_name==header_var.var_name{
                    //here ---
                    var_exist=true;
                    header_var.var_is_used=true;
                    if !header_var.var_is_assing{
                        // **print**
                        println!("* * * * --> {}: variable {} is used uninitialized",header_var.var_line,header_var.var_name);
                    }
                }
                if header_vars_len<=0 || var_exist{
                    break;
                }else{
                    header_vars_len-=1;
                }
            }
        }
        if !self.vars.is_empty() && !var_exist{
            let mut body_vars_len=self.vars.len()-1;
            loop {
                let body_var = &mut self.vars[body_vars_len];
                if var_name==body_var.var_name{
                    // here ---
                    var_exist=true;
                    body_var.var_is_used=true;
                    if !body_var.var_is_assing {
                        println!("* * * * --> {}: variable {} is used uninitialized",body_var.var_line,body_var.var_name);
                    }
                }
                if body_vars_len<=0 || var_exist{
                    break;
                }else{
                    body_vars_len-=1;
                }
            }
        }
        var_exist
    }

    fn check_vars_exist(&mut self,var_name: &str) -> bool{

        if self.check_header_vars(var_name){
            return true
        }
        if self.check_body_vars(var_name){
            return true
        }
        false
    }

    fn check_header_vars(&mut self, var_name:&str) -> bool{
        let mut var_exist=false;
        if !self.header_vars.is_empty(){
            let mut header_vars_len=self.header_vars.len()-1;
            loop {
                let header_var = &self.header_vars[header_vars_len];
                //println!("header var '{}' type is '{}' line is {} ",header_var.var_name, header_var.var_type, header_var.var_line);
                if var_name==header_var.var_name{
                    var_exist=true;
                }
                if header_vars_len<=0 || var_exist{
                    break;
                }else{
                    header_vars_len-=1;
                }
            }
        }
        var_exist 
    }

    fn check_body_vars(&self, var_name:&str) -> bool{
        let mut var_exist=false;
        if !self.vars.is_empty(){
            let mut body_vars_len=self.vars.len()-1;
            loop {
                let body_var = &self.vars[body_vars_len];
                //println!("body var '{}' type is '{}' line is {} ",body_var.var_name, body_var.var_type, body_var.var_line);
                if var_name==body_var.var_name{
                    var_exist=true;
                }
                if body_vars_len<=0 || var_exist{
                    break;
                }else{
                    body_vars_len-=1;
                }
            }
            if var_exist {
            }
        }
        var_exist 
    }

    fn create_var(&mut self, var_name:String, var_type:String, var_value:String, is_header: bool,)  {
        if is_header {
            unsafe{
                self.header_vars.push(
                    VarStruct::construct(var_value, var_name, var_type, LINE+1, false, false)
                );
            }
        }else{
            unsafe{
                self.vars.push(
                    VarStruct::construct(var_value, var_name, var_type, LINE+1, false, false)
                );
            }
        }
    }
}

fn nested_assing_var(nested_scops : &mut Vec<BodyStruc>, var_name:&str,  var_value:&str){
    //println!("here nested using var");
    let mut var_exist = false;
    if !nested_scops.is_empty(){
        let mut nested_scops_len = nested_scops.len() -1;
        loop{
            //println!("checking scope {} ... ",nested_scops_len);
            let current_scope = &mut nested_scops[nested_scops_len];
            var_exist = current_scope.assing_var(var_name,var_value);
            if nested_scops_len<=0 || var_exist{
                break;
            }else {
                nested_scops_len-=1;
            }
        }
        //println!("");
    }
    if !var_exist{
        unsafe{
            println!("* * * * --> {}: assingment '{}' faild , var does not exist!",LINE,var_name)
        }
    }else{
        //println!("assingment var '{}' finished!",var_name)
    }
}

fn nested_using_var(nested_scops : &mut Vec<BodyStruc>, var_name:&str){
    //println!("here nested using var");
    let mut var_exist = false;
    if !nested_scops.is_empty(){
        let mut nested_scops_len = nested_scops.len() -1;
        loop{
            //println!("checking scope {} ... ",nested_scops_len);
            let current_scope = &mut nested_scops[nested_scops_len];
            var_exist = current_scope.using_var(var_name);
            if nested_scops_len<=0 || var_exist{
                break;
            }else {
                nested_scops_len-=1;
            }
        }
        //println!("");
    }
    if !var_exist{
        unsafe{
            println!("* * * * --> {}: use '{}' faild , var does not exist!",LINE,var_name)
        }
    }else{
    //    println!("useding var {} finished !",var_name)
    }
}

fn nested_check_var_exist( nested_scops : &mut Vec<BodyStruc>, var_name:&str) -> bool{
    //println!("in nested_check_var_exist func ... ");
    let mut var_exist = false;
    if !nested_scops.is_empty(){
        let mut nested_scops_len = nested_scops.len() -1;
        loop{
            //println!("checking scope {} ... ",nested_scops_len);
            let current_scope = &mut nested_scops[nested_scops_len];
            var_exist = current_scope.check_vars_exist(var_name);
            if nested_scops_len<=0 || var_exist{
                break;
            }else {
                nested_scops_len-=1;
            }
        }
        //println!("");
    }
    var_exist
}

fn nested_create_var(nested_scops : &mut Vec<BodyStruc>, var_name:String,var_type:String , is_header: bool){
    let var_exist = nested_check_var_exist(nested_scops,var_name.as_str());
    if !var_exist{
        let nested_scops_len = nested_scops.len() -1;
        // create var
        nested_scops[nested_scops_len].create_var(var_name.to_string(), var_type.to_string(), "".to_string(), is_header);
        //println!("var {} type of {} created !\n",var_name,var_type);
    }else{
        unsafe{println!("! ! ! ! --> {}: variable {} exist in this scops, cant create",LINE,var_name);}
    }
}

fn check_func_value (func_list:&mut Vec<BodyStruc>, func_name:&str,  vars_len: i32) -> u8{
    let mut is_mathch=false;
    let mut func_exist=false;

    if !func_list.is_empty(){
        let mut func_list_len = func_list.len()-1;
        loop {
            let func=&func_list[func_list_len];
            if func.func_name==func_name {
                func_exist=true;
                
                if !func.header_vars.is_empty() {
                    //check if arguman is OK oe NOT
                    if !(vars_len == func.header_vars.len() as i32 -1){
                    }else{
                        is_mathch=true;
                    }
                    break;
                }
                else{
                    // ------------ maybe does not work ------------
                    if vars_len!=0{
                    }
                }
            }
            if func_list_len<=0{
                break;
            }
            else {
                func_list_len-=1;
            }
        }
    }
    if is_mathch{
        // func is exist and flist_len is OK
        return 0
    }
    else if func_exist {
        // func is exist but flist_len not OK
        return 1
    }
    // func does not exist
    return 2
}

fn destroy_last_nested_scops(nested_scops : &mut Vec<BodyStruc>){
    //println!("- - - - destriy the {}(last) BodyStruct from nested_scope",nested_scops.len());
    let nested_scops_len=nested_scops.len()-1;
    if !nested_scops[nested_scops_len].header_vars.is_empty(){
        let mut scop_header_vars_len=nested_scops[nested_scops_len].header_vars.len()-1;
        loop{
            let header_var=&nested_scops[nested_scops_len].header_vars[scop_header_vars_len];
            //println!("in header variable {} : var_is_assing = {} , var_is_used = {}",header_var.var_name,header_var.var_is_assing,header_var.var_is_used);
            if !header_var.var_is_used {// && !header_var.var_is_assing{
                println!("* * * * --> {}: variable {}  is defined but never used!",header_var.var_line,header_var.var_name);
            }
            if scop_header_vars_len<=0{
                break;
            }
            scop_header_vars_len-=1;
        }
    }
    if !nested_scops[nested_scops_len].vars.is_empty(){
        let mut scop_body_vars_len=nested_scops[nested_scops_len].vars.len()-1;
        loop{
            let body_var=&nested_scops[nested_scops_len].vars[scop_body_vars_len];
            //println!("in body variable {} : var_is_assing = {} , var_is_used = {}",body_var.var_name,body_var.var_is_assing,body_var.var_is_used);
            if !body_var.var_is_used{
                println!("* * * * --> {}: variable {}  is defined but never used!",body_var.var_line, body_var.var_name);
            }
            if scop_body_vars_len<=0{
                break;
            }
            scop_body_vars_len-=1;
        }
    }
    nested_scops.pop();
}

fn copy_lase_scope_to_func(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>){
    if !nested_scops.is_empty(){
        let nested_scope_len=nested_scops.len()-1;
        let scop_func = &nested_scops[nested_scope_len];
        let mut body_str= BodyStruc::construct(true, nested_scops[nested_scope_len].func_name.clone(), nested_scops[nested_scope_len].func_type.clone());
        if !scop_func.header_vars.is_empty(){
            let mut header_len = scop_func.header_vars.len()-1;
            loop{
                body_str.create_var(
                    scop_func.header_vars[header_len].var_name.clone(),
                    scop_func.header_vars[header_len].var_type.clone(),
                    scop_func.header_vars[header_len].var_value.clone(),
                    true
                );
                body_str.assing_var(scop_func.header_vars[header_len].var_name.as_str(), "");
                if header_len<=0{
                    break
                }
                header_len-=1;
            }
        }
        // copy header vars ...
        // don't need now
        func_list.push(body_str);
        //println!("last nested scope copied to func_list\n");

    }
}
// parser ***********************************************************************************************************************************

fn is_key (token:&str) -> bool {
    if 
        token == "def" || token == "num" || token == "list"||
        token == "return" || token == "if" || token == "while" ||
        token == "else" ||  token == "for" || token == "break"
        // || token == "numread" || token == "makelist"
        // || token == "numprint" || token == "listlen" || token == "exit"
        {return true}
    false
    // key
}

fn is_type(token:&str) ->bool{
    token=="list" || token=="num"
    //type
}

fn flist(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>,mut flist_len:i32) ->i32{
    let mut token:(String,String,i8);
    
    token=nexttoken();

    // flist ::= <iden> ... 
    // first flist = <iden>
    // van be is_type
    if token.1=="id"{
        if is_key(token.0.as_str()) {
            unsafe {println!("! ! ! ! --> {}: flist <iden> shuld not be a key word '{}'",LINE+1,token.0);}
        }
        droptoken();
        
        // id for nested_scops[0]
        let var_name = token.0; 

        // flist ::= <iden> : <type> ...
        token=nexttoken();
        droptoken();
        if !(token.0==":"){unsafe {println!("! ! ! ! --> {}: expected ':' found '{}'",LINE+1,token.1);}}

            
        // flist ::= ... <type> ...
        token=nexttoken();
        droptoken();
        if !(is_type(&token.0))
        {unsafe {println!("! ! ! ! --> {}: expected <type> found <{}>",LINE+1,token.1);}}

            
        // id for nested_scops[0]
        let var_type=token.0; 

        // flist ::= iden : type , flist
        // if next token is "," then call flist
        // else just return
        token=nexttoken();

        // add this variable to nested_scops[0] func header
        nested_create_var(nested_scops ,var_name.clone(), var_type, true);
        nested_assing_var(nested_scops, var_name.as_str(),"");
        // increase var len
        flist_len+=1;
        if token.0==","{
            droptoken();
            flist_len=flist(func_list,nested_scops,flist_len);
        } else {
            
        }

    }else {

    }
    flist_len
}

fn clist(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>,mut clist_len:u16) -> u16{
    //<clist> ::= <expr> | <expr> , <clist> | E
    let mut token:(String,String,i8);
    token=nexttoken();
    
    // first expr
    if  token.0=="+" || token.0=="++" || token.0=="-" || token.0=="--" ||
        token.0=="!" || token.0=="(" || token.1=="num" || token.1=="id" {
        
        clist_len+=1;

        let expr_token = expr(func_list,nested_scops);
        
        if expr_token.1=="id"{
            nested_using_var(nested_scops, expr_token.0.as_str());
        }

        token=nexttoken();
        if token.0==","{
            droptoken();
            clist_len = clist(func_list,nested_scops,clist_len);
        } 
    }
    clist_len 
}

fn expr(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr>::= <expr1><expr_prime>
    let expr1_token = expr1(func_list,nested_scops);

    let exprp_token = expr_prime(func_list,nested_scops);
    
    
    // token badi expri_token '=' bashe
    // yani : <iden>=<vale>
    // expr1_token bayad meghdar begire :: nested_assing_var
    if exprp_token.0{
        // age <iden> nabod khata bde
        if expr1_token.1=="id"{
            if !is_key(expr1_token.0.as_str()){
                nested_assing_var(nested_scops, expr1_token.0.as_str(), exprp_token.1.as_str());
                nested_using_var(nested_scops, expr1_token.0.as_str())
            }else{
                unsafe{
                    println!("! ! ! ! --> {}: expected <iden> found <{}>",LINE+1,expr1_token.1);
                }
            }
        }else{
            unsafe{
                println!("! ! ! ! --> {}: variable shuld not be a key word '{}'",LINE+1,expr1_token.1);
            }
        }
        return (expr1_token.0,expr1_token.1)
    }else{
        expr1_token
    }
}
fn expr_prime(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (bool,String,String){
    // <expr_prime>::= '=' <expr1><expr_prime> | E

    let token:(String,String,i8);
    
    token = nexttoken();
    if token.0=="="{
        droptoken();

        let expr1_token = expr1(func_list,nested_scops);

        // id = id
        // age samt rast '=' <iden> bod bayad tabe use farakhoni shavad
        if expr1_token.1=="id"{
            nested_using_var(nested_scops, expr1_token.0.as_str())
        }

        // dont have id=id=id...
        let exprp_token = expr_prime(func_list,nested_scops);
        if exprp_token.0{
            unsafe{
                println!("! ! ! ! --> {}: expected Everything else '='! , dont have id=id=id",LINE+1);
            }
        }
        return (true,expr1_token.0,expr1_token.1)
    }else {
        return (false,"".to_string(),"".to_string())
    }
    // age '=' pazirofte shode ba
}

fn expr1(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr1>::= <expr2><expr1_prime>
    let expr2_token = expr2(func_list,nested_scops);
    
    let expr1p_token = expr1_prime(func_list,nested_scops);

    if expr1p_token.0{
        // bayad hasel id || id || ... ro bargardone
        // inja be ("".to_string() ,"num".to_string()) ktefa mikonim
        // bayad tabe 
        if expr2_token.1=="id" {
            nested_using_var(nested_scops, expr2_token.0.as_str());
        }
        return ("".to_string() ,"num".to_string())
    }else{
        expr2_token
    }
}
fn expr1_prime(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (bool,String,String){
    // <expr_prime1>::= '||' <expr2><expr_prime1> | E

    let token:(String,String,i8);

    token = nexttoken();

    if token.0=="||"{
        droptoken();

        let expr2_token = expr2(func_list,nested_scops);
        if expr2_token.1=="id"{
        //nested_assing_var(nested_scops, expr2_token.0.as_str()," ".to_string().as_str())
            nested_using_var(nested_scops , expr2_token.0.as_str())

        }
        expr1_prime(func_list,nested_scops);
        
        (true,expr2_token.0,expr2_token.1)
    }
    else {
        (false,"".to_string(),"".to_string())
    }
}

fn expr2(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr2>::= <expr3><expr2_prime>
    let expr3_token = expr3(func_list,nested_scops);
    let expr2p_token = expr2_prime(func_list,nested_scops);
    if expr2p_token.0{
        if expr3_token.1=="id"{
        // bayad hasel id && id && ... ro bargardone
        // inja be ("".to_string() ,"num".to_string()) ktefa mikonim
        // bayad tabe 
            //nested_assing_var(nested_scops, expr3_token.0.as_str() , " ")
            nested_using_var(nested_scops , expr3_token.0.as_str());
        }
        (expr3_token.0,"num".to_string())
    }else{
        expr3_token
    }
}
fn expr2_prime(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (bool,String,String){
    // <expr2_prime>::= '&&' <expr3><expr2_prime> | E

    let token:(String,String,i8);

    token = nexttoken();
    if token.0=="&&"{
        droptoken();

        let expr3_token = expr3(func_list,nested_scops);
        if expr3_token.1=="id"{
            nested_using_var(nested_scops, expr3_token.0.as_str());
        }
        expr2_prime(func_list,nested_scops);
        (true,expr3_token.0,expr3_token.1)
    }
    else {
        (false,"".to_string(),"".to_string())
    }
}

fn expr3(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr3>::= <expr4><expr3_prime>
    let expr4_token = expr4(func_list,nested_scops);
    let expr3p_token = expr3_prime(func_list,nested_scops);
    if expr3p_token.0{
        if expr4_token.1=="id"{
        // bayad hasel id == id ya id != id ... ro bargardone
        // inja be ("".to_string() ,"num".to_string()) ktefa mikonim
        // bayad tabe 
            nested_using_var(nested_scops , expr4_token.0.as_str())
        }
        (expr4_token.0,"num".to_string())
    }else{
        expr4_token
    }
}
fn expr3_prime(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (bool,String,String){
    // <expr3_prime>::= '==' '!=' <expr4><expr3_prime> | E

    let token:(String,String,i8);

    token = nexttoken();
    
    if token.0=="=="{
        droptoken();
        
        let expr4_token = expr4(func_list,nested_scops);
        if expr4_token.1=="id"{
            nested_using_var(nested_scops, expr4_token.0.as_str());
        }
        expr3_prime(func_list,nested_scops);
        (true,expr4_token.0,expr4_token.1)
    }
    else if token.0=="!="{
        droptoken();
        
        let expr4_token = expr4(func_list,nested_scops);
        if expr4_token.1=="id"{
            nested_using_var(nested_scops, expr4_token.0.as_str());
        }
        expr3_prime(func_list,nested_scops);
        (true,expr4_token.0,expr4_token.1)
    }
    else {
        (false,"".to_string(),"".to_string())
    }
}

fn expr4(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr4>::= <expr5><expr4_prime>
    let expr5_token = expr5(func_list,nested_scops);
    let expr4p_token = expr4_prime(func_list,nested_scops);
    if expr4p_token.0{
        if expr5_token.1=="id"{
        // bayad hasel id < > <= >= id ... ro bargardone
        // inja be ("".to_string() ,"num".to_string()) ktefa mikonim
        // bayad tabe 
            nested_assing_var(nested_scops, expr5_token.0.as_str() , " ")
        }
        (expr5_token.0,"num".to_string())
    }else{
        expr5_token
    }
}
fn expr4_prime(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (bool,String,String){
    // <expr4_prime>::= '<' '<=' '>' '>=' <expr5><expr4_prime> | E

    let token:(String,String,i8);
    token = nexttoken();
    let mut expr5_token:(String,String)=("".to_string(),"".to_string());
    let mut epr4=false;
    if token.0=="<"{
        epr4=true;
        droptoken();
        expr5_token = expr5(func_list,nested_scops);
        expr4_prime(func_list,nested_scops);
    }
    else if token.0=="<="{
        epr4=true;
        droptoken();
        expr5_token = expr5(func_list,nested_scops);
        expr4_prime(func_list,nested_scops);
    }
    else if token.0==">"{
        epr4=true;
        droptoken();
        expr5_token = expr5(func_list,nested_scops);
        expr4_prime(func_list,nested_scops);
    }
    else if token.0==">="{
        epr4=true;
        droptoken();
        expr5_token = expr5(func_list,nested_scops);
        expr4_prime(func_list,nested_scops);
    }
    else {}

    // age yek shart bar gharar bod ...
    if epr4==true{
        if expr5_token.1=="id"{
            nested_using_var(nested_scops, expr5_token.0.as_str());
        }
        (true,"".to_string(),"num".to_string())
    }else{
        (false,"".to_string(),"".to_string())
    }
}

fn expr5(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr5>::= <expr6><expr5_prime>
    let expr6_token = expr6(func_list,nested_scops);
    let expr5p_token = expr5_prime(func_list,nested_scops);
    if expr5p_token.0{
        if expr6_token.1=="id"{
        // bayad hasel id < > <= >= id ... ro bargardone
        // inja be ("".to_string() ,"num".to_string()) ktefa mikonim
        // bayad tabe 
        //nested_assing_var(nested_scops, expr6_token.0.as_str() , " ");
            nested_using_var(nested_scops , expr6_token.0.as_str());

        }
        (expr6_token.0,"num".to_string())
    }else{
        expr6_token
    }
}
fn expr5_prime(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (bool,String,String){
    // <expr5_prime>::= '-' '+' <expr6><expr5_prime> | E

    let token:(String,String,i8);
    token = nexttoken();

    let mut expr6_token:(String,String)=("".to_string(),"".to_string());
    let mut epr5=false;
    if token.0=="+"{
        epr5=true;
        droptoken();
        expr6_token = expr6(func_list,nested_scops);
        expr5_prime(func_list,nested_scops);
    }
    else if token.0=="-"{
        epr5=true;
        droptoken();
        expr6_token = expr6(func_list,nested_scops);
        expr5_prime(func_list,nested_scops);
    }
    else {}

    if epr5 {
        if expr6_token.1=="id"{
            nested_using_var(nested_scops, expr6_token.0.as_str());
        }
        (true,"".to_string(),"num".to_string())
    }else{
        (false,"".to_string(),"".to_string())
    }
}

fn expr6(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr6>::= <expr7><expr6_prime>
    let expr7_token = expr7(func_list,nested_scops);
    let expr6p_token = expr6_prime(func_list,nested_scops);
    if expr6p_token.0{
        if expr7_token.1=="id"{
        // bayad hasel id < > <= >= id ... ro bargardone
        // inja be ("".to_string() ,"num".to_string()) ktefa mikonim
        // bayad tabe 
            //nested_assing_var(nested_scops, expr7_token.0.as_str() , " ");
            nested_using_var(nested_scops , expr7_token.0.as_str());
        }
        (expr7_token.0,"num".to_string())
    }else{
        expr7_token
    }
}
fn expr6_prime(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (bool,String,String){
    // <expr6_prime>::= '/' '%' '*' <expr7><expr6_prime> | E

    let token:(String,String,i8);

    token = nexttoken();
    let mut expr7_token:(String,String)=("".to_string(),"".to_string());
    let mut epr6=false;
    if token.0=="*"{
        epr6=true;
        droptoken();
        expr7_token = expr7(func_list,nested_scops);
        expr6_prime(func_list,nested_scops);
    }
    else if token.0=="%"{
        epr6=true;
        droptoken();
        expr7_token = expr7(func_list,nested_scops);
        expr6_prime(func_list,nested_scops);
    }
    else if token.0=="/"{
        epr6=true;
        droptoken();
        expr7_token = expr7(func_list,nested_scops);
        expr6_prime(func_list,nested_scops);
    }
    else {}

    if epr6 {
        if expr7_token.1=="id"{
            nested_using_var(nested_scops, expr7_token.0.as_str());
        }
        (true,"".to_string(),"num".to_string())
    }else{
        (false,"".to_string(),"".to_string())
    }
}

fn expr7(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr7>::= <expr8> ** <expr7>|<expr8>
    let token:(String,String,i8);
    let expr8_token = expr8(func_list,nested_scops);
    token = nexttoken();
    if token.0=="**"{
        if expr8_token.1=="id"{
            nested_using_var(nested_scops, expr8_token.0.as_str())
        }
        droptoken();
        expr7(func_list,nested_scops);
        return (expr8_token.0,"num".to_string())
    }
    else {
        expr8_token
    }
}

fn expr8(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    // <expr8>::= !|+|++|-|-- <expr9>
    let token:(String,String,i8);
    token=nexttoken();
    let mut expr9_token:(String,String)=("".to_string(),"".to_string()); 
    
    if  token.0=="-" || token.0=="--" || token.0=="+" || token.0=="++" || token.0=="!"{
    
        
        if token.0=="!"{
            droptoken();
            expr9_token = expr9(func_list,nested_scops);
        } else if token.0=="-"{
            
            droptoken();
            expr9_token = expr9(func_list,nested_scops);
        } else if token.0=="+"{
            
            droptoken();
            expr9_token = expr9(func_list,nested_scops);
        } else if token.0=="++"{
            
            droptoken();
            expr9_token = expr9(func_list,nested_scops);
        } else if token.0=="--"{
            
            droptoken();
            expr9_token =expr9(func_list,nested_scops);
        }

        // ---
        if expr9_token.1=="id"{
            nested_using_var(nested_scops, expr9_token.0.as_str())
        }
        // chon + ++ - -- meghdar var ra taghir midahand , bayad khod var bargasht dade shavad
        expr9_token
    }
    else{
        expr9(func_list,nested_scops)
    }
}

fn expr9(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>) -> (String,String){
    let mut token:(String,String,i8);
    token=nexttoken();
    let mut return_vlaue:(String,String)=("".to_string(),"".to_string());
    // <expr9>::= (<expr>)
    if token.0=="("{
        droptoken();

        // i realy dont know if it's works ! ***********
        let expr_token = expr(func_list,nested_scops);

        token=nexttoken();
        droptoken();
        if token.0==")"{
            
        }
        else{unsafe{ println!("! ! ! ! --> {}: expected ')' found {}",LINE+1,token.0)}}
        
        // --
        return expr_token
    }
    else if token.1=="id" {

        return_vlaue=(token.0,token.1);
        
        droptoken();
        token=nexttoken();
        // <expr9>::= <iden> (<clist>)
        if token.0=="("{
            
            droptoken();
            
            let clist_len = clist(func_list,nested_scops,0);
            
            token=nexttoken();
            droptoken();
            if token.0==")"{
                let func_check_code = check_func_value(func_list, return_vlaue.0.as_str(), clist_len as i32 -1);
                if func_check_code==2{
                    unsafe {println!("! ! ! ! --> {}: func '{}' does not exist",LINE+1,return_vlaue.0);}
                }else if func_check_code==1{
                    unsafe {println!("! ! ! ! --> {}: func is exist but flist_len not mathc {}",LINE+1,clist_len-1);}
                }else if func_check_code==0{
                    // nothing - OK
                }
            }
            else{ unsafe {println!("! ! ! ! --> {}: expected ) found {}",LINE+1,token.0)}}
            // --
            return ("".to_string(), "num".to_string())
        }

        else if token.0=="["{
            droptoken();
            
            expr(func_list,nested_scops);

            token=nexttoken();
            droptoken();
            if token.0=="]"{

            }
            else{unsafe{println!("! ! ! ! --> {}: expected ']' found '{}'",LINE+1,token.0)}}
            
        }
        // <expr9>::= <iden> (<expr9>)
        if token.0=="++" {
            if is_key(return_vlaue.0.as_str()){
                unsafe {
                    println!("! ! ! ! --> {}: var {} shuld not be a key word",LINE+1,return_vlaue.0);
                }
            }
            nested_using_var(nested_scops, return_vlaue.0.as_str());
            droptoken();
            // return <iden>
            return return_vlaue
        }
        else if token.0=="--" {
            if is_key(return_vlaue.0.as_str()){
                unsafe {
                    println!("! ! ! ! --> {}: var {} shuld not be a key word",LINE+1,return_vlaue.0);
                }
            }
            nested_using_var(nested_scops, return_vlaue.0.as_str());
            droptoken();
            // return <iden>
            return return_vlaue
        }
        else{
            if is_key(return_vlaue.0.as_str()){
                unsafe {
                    println!("! ! ! ! --> {}: var {} shuld not be a key word",LINE+1,return_vlaue.0);
                }
            }
            // return <iden>
            return return_vlaue
        }
    }
    else if token.1=="num"{
        droptoken();
    }
    return_vlaue
}

fn defvar(nested_scops:&mut Vec<BodyStruc>){
    let mut token:(String,String,i8);
    
    token=nexttoken();
    droptoken();
    if !(token.0=="var")
    {unsafe {println!("! ! ! ! --> {}: expected 'var' found '{}'",LINE+1,token.0);}}


    token = nexttoken();
    droptoken();
    if !(token.1=="id")
    {unsafe {println!("! ! ! ! --> {}: expected <iden> found <{}>",LINE+1,token.1);}}


    if is_key(token.0.as_str()){
        unsafe {
            println!("! ! ! ! --> {}: var {} shuld not be a key word",LINE+1,token.0);
        }
    }
    let var_name=token.0;
    token = nexttoken();
    droptoken();
    if !(token.0==":")
    {unsafe {println!("! ! ! ! --> {}: expected ':' found '{}'",LINE+1,token.0);}}


    
    token = nexttoken();
    droptoken();
    if !(is_type(&token.0)) 
    {unsafe {println!("! ! ! ! --> {}: expected <type> found <{}>",LINE+1,token.1);}}


    let var_type=token.0;
    // creating variale 
    //println!("- - - - creating var name {} type of {}",var_name,var_type);
    nested_create_var(nested_scops, var_name, var_type, false);
}

fn stmt(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>,is_for_loop:bool){
    let mut token:(String,String,i8);
    
    token=nexttoken();
    
    // <stmt> = if ( expr ) stmt else stmt
    if token.0=="if"{
        droptoken();

        token=nexttoken();
        droptoken();
        
        if !(token.0=="(")
        {unsafe {println!("! ! ! ! --> {}: expected '(' found '{}'",LINE+1,token.0);}}
        expr(func_list,nested_scops);
        token=nexttoken();
        droptoken();
        
        
        if !(token.0==")") 
        {unsafe {println!("! ! ! ! --> {}: expected ')' found '{}'",LINE+1,token.0);}}
        stmt(func_list,nested_scops,false);
        token=nexttoken();
        
        if token.0=="else"{
            droptoken();
            stmt(func_list,nested_scops,false);
        }
        else {
        //    unsafe {println!("expected 'else' found '{}' in line {}",token.0,LINE+1);}
        }
    }

    // <stmt> = for ( iden in expr ) stmt 
    else if token.0=="for"{
        droptoken();

        // creating BodyStruct and add to nested_scops
        let body_str= BodyStruc::construct(false, "".to_string(),"".to_string());
        nested_scops.push(body_str);
        //println!("- - - - add new BodyStruct to nested_scops number of BodyStructs={}, for for loops",nested_scops.len());

        token=nexttoken();
        droptoken();
        if !(token.0=="(")
        {unsafe {println!("! ! ! ! --> {}: expected '(' found '{}'",LINE+1,token.0);}}
        

            
        token=nexttoken();
        droptoken();
        if !(token.1=="id")
        {unsafe {println!("! ! ! ! --> {}: expected <iden> found <{}>",LINE+1,token.1);}}
        
        

        if is_key(token.0.as_str())
        {unsafe{println!("! ! ! ! --> {}: var {} shuld not be a key word",LINE+1,token.0);}}
        //let nested_scops_len = nested_scops.len()-1;
        // add for header var to last nested_scops
        if nested_check_var_exist(nested_scops, token.0.as_str()){
            // if var is exist assing
            nested_assing_var(nested_scops, token.0.as_str(), "");
        }else{
            // if var does not exist create
            nested_create_var(nested_scops, token.0, "num".to_string(), true);
        }
        

        token=nexttoken();
        droptoken();
        if !(token.0=="in")
        {unsafe {println!("! ! ! ! --> {}: expected 'in' found '{}'",LINE+1,token.0);}}



        let expr_token = expr(func_list,nested_scops);
        if !(expr_token.1=="id")
        {unsafe{println!("! ! ! ! --> {}: expected <iden> found '{}'",LINE+1,expr_token.1);}}
        nested_using_var(nested_scops, expr_token.0.as_str());
        
        
        token=nexttoken();
        droptoken();
        if !(token.0==")"){unsafe {println!("! ! ! ! --> {}: expected ')' found '{}'",LINE+1,token.0,);}}
        stmt(func_list,nested_scops,true);

        // * * * * * * * *  destroy lest nested_scops
        //println!("- - - - destroy last BodyStruct from nested_scops that added for for loops");
        destroy_last_nested_scops(nested_scops);
    }
    
    // <stmt> = while ( <expr> ) <stmt> 
    else if token.0=="while"{
        droptoken();
        // (
        token=nexttoken();
        droptoken();
        if token.0=="("{unsafe {println!("! ! ! ! --> {}: expected '(' found '{}'",LINE+1,token.0);}}

        // <expr>
        expr(func_list,nested_scops);

        token=nexttoken();
        droptoken();
        if token.0==")"
        {unsafe {println!("! ! ! ! --> {}: expected ')' found '{}'",LINE+1,token.0);}}

        // <stmt>
        stmt(func_list,nested_scops,false);
    }
    
    // <stmt> ::= return <expr> ;
    else if token.0=="return"{
        droptoken();
        
        expr(func_list,nested_scops);
        token=nexttoken();
        droptoken();
        if !(token.0==";"){
            unsafe {println!("! ! ! ! --> {}: expected ';' fount '{}'",LINE+1,token.0);}
        }
    }

    // <stmt> ::= {<body>}
    else if token.0=="{"{
        if !is_for_loop{
            let body_str= BodyStruc::construct(false, "".to_string(),"".to_string()); 
            nested_scops.push(body_str);
            //println!("- - - - add new BodyStruct to nested_scops number of BodyStructs={}, for {{ <body> }}",nested_scops.len());
        }
        // <body>
        droptoken();
        body(func_list,nested_scops);
        // }
        token=nexttoken();
        droptoken();
        if ! (token.0=="}"){
            unsafe { println!("! ! ! ! --> {}: expected '}}' found '{}'",LINE+1,token.0);}
        }
        if !is_for_loop{
            //println!("- - - - destroy last BodyStruct from nested_scops that added for {{ <body> }}");
            destroy_last_nested_scops(nested_scops);
        }
    }
    // <stmt>::= <defvar> (first defvar);
    else if token.0=="var"{
    
        defvar(nested_scops);
        
        token=nexttoken();
        droptoken();
        if !(token.0==";") {unsafe {println!("! ! ! ! --> {}: expected ';' found '{}' ",LINE+1,token.0);}}
    }
    
        // <stmt>::= <expr> ; (first expr)
    else if token.1=="id" || token.1=="num" || token.0=="-" || token.0=="+" || token.0=="--"|| token.0=="++" || token.0=="!" || token.0=="(" {
        
        expr(func_list,nested_scops);
        
        token=nexttoken();
        droptoken();
        if !(token.0==";") {unsafe {println!("! ! ! ! --> {}: expected ';' found '{}'",LINE+1,token.0);}}
    }else
    {
        unsafe{ println!("! ! ! ! --> {}: unexpected token {}",LINE+1,token.0);}
    }
}

fn body(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>){
    //body ::= stmt | stmt body
    //println!("<body> ::= <stmt> descovering");
    stmt(func_list,nested_scops,false);
    let token:(String,String,i8);
    token=nexttoken();
    // first body = stmt first = ...
    if  token.0=="if" || token.0=="for" || token.0=="while" || token.0=="return" || token.0=="{" || token.0=="var" || token.1=="id" || 
        token.1=="num" || token.0=="!" || token.0=="-" || token.0=="(" || token.0=="+"  || token.0=="--"  || token.0=="++"
    {

        body(func_list,nested_scops);
    }
}

fn func(func_list:&mut Vec<BodyStruc>, nested_scops:&mut Vec<BodyStruc>){
    let mut token:(String,String,i8);
    
    // func ::= "def" ...
    token=nexttoken();
    droptoken();
    if !(token.0=="def")
    {unsafe {println!("! ! ! ! --> {}: expected 'def' found '{}'",LINE+1,token.0);}}    
    
    
    // func ::= def <iden> ...
    token=nexttoken();
    droptoken();
    if !(token.1=="id")
    {unsafe {println!("! ! ! ! --> {}: expected <iden> found <{}>",LINE+1,token.1);}}
    if is_key(token.0.as_str()) 
    {unsafe {println!("! ! ! ! {}: --> def <iden> shuld not be a key word '{}'",LINE+1,token.0);}}
    let func_name = token.0;
    
    
    // func ::= ... "(" ...
    token=nexttoken();
    droptoken();
    if !(token.0=="(")
    {unsafe {println!("! ! ! ! --> {}: expected '(' found '{}'",LINE+1,token.0);}}
    
    // func ::= ... <flist> ...                
    let flist_len = flist(func_list,nested_scops,0);
    
    let func_check_code = check_func_value(func_list, func_name.as_str(), flist_len);
    if func_check_code!=2{
    unsafe{println!("\n! ! ! ! --> {}: function '{}' is alrady axist",LINE+1,func_name);}}
    
    // set func name for sested_scope[0] 
    nested_scops[0].func_name=func_name;


    // func ::= ... ")" ...
    token=nexttoken();
    droptoken();
    if !(token.0==")")
    {unsafe {println!("! ! ! ! --> {}: expected ')' found '{}'",LINE+1,token.0);}}

    
    // func ::= ... ":" ...
    token=nexttoken();
    droptoken();
    if !(token.0==":")
    {unsafe {println!("! ! ! ! --> {}: expected ':' found '{}'",LINE+1,token.0);}}

    
    // func ::= ... <type> ...
    token=nexttoken();
    // is_type
    if !(is_type(&token.0))
    {unsafe {println!("! ! ! ! --> {}: expected <type> found <{}>",LINE+1,token.1);}}
    // set function name 
    nested_scops[0].func_type=token.0;
    droptoken();

    
    // func ::= ... "{" ...
    token=nexttoken();
    droptoken();

    // copy current function to func_list
    copy_lase_scope_to_func(func_list,nested_scops);

    // !we dont create an ather BosySrtruct for this body
    if !(token.0=="{")
    {unsafe {println!("! ! ! ! --> {}: expected '{{' found '{}",LINE+1,token.0);}}

    
    // func ::= ... <body> ...
    body(func_list,nested_scops);

    // func ::= ... "}" ...
    token=nexttoken();
    droptoken();
    if token.0=="}"{

    }
    else {unsafe {println!("! ! ! ! --> {}: expected '}}' found '{}'",LINE+1,token.0);}}
}

fn prog(func_list:&mut Vec<BodyStruc>){

    let mut nested_scops:Vec<BodyStruc>=Vec::new();
    // proc ::= <func> | <func> <proc>
    //println!("- - - - add new BodyStruct to nested_scops number of BodyStructs={}, for <func>",nested_scops.len());

    let body_str= BodyStruc::construct(true, "".to_string(),"".to_string()); 
    nested_scops.push(body_str);
    nested_scops[0].is_func=true;
    func(func_list,&mut nested_scops);

    //println!("- - - - destroy last BodyStruct from nested_scops that added for <func>");
    //func_list.push(nested_scops[0]);
    // assing nested_scops[0] to func list 
    // --

    // check nested_scope[0]    
    destroy_last_nested_scops(&mut nested_scops);

    let token:(String,String,i8);
    token= nexttoken();
    // first prog = def
    if token.0=="def"{
        prog(func_list);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    //get input file
    
    //let stdin = io::stdin(); // We get `Stdin` here.
    //println!("{:#?}",stdin);

    let filename = &args[1];
    let mut content = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    //input file content into a Vector
    unsafe{
        CHAR_VEC= content.to_string().chars().collect();
        MAX=content.len() as i32;
    }
    content.clear();

    let mut func_list:Vec<BodyStruc> = Vec::new();

    let mut body_str:BodyStruc;
    // makelist(n) func
    body_str= BodyStruc::construct(true, "makelist".to_string(),"list".to_string()); 
    body_str.header_vars.push(
        // makelist(n) argumant :  n (num) 
        VarStruct::construct(
            "".to_string(),"n".to_string(),"num".to_string(),0,true,true
        )
    );
    // add to func list
    func_list.push(body_str);

    // numprint(n) func
    body_str=  BodyStruc::construct(true, "numprint".to_string(),"void".to_string()); 
    body_str.header_vars.push(
        // numprint(n) argumant : n (num)
        VarStruct::construct(
            "".to_string(),"n".to_string(),"num".to_string(),0,true,true
        )
    );
    // add to func list
    func_list.push(body_str);

    
    // listlen(v) func
    body_str=  BodyStruc::construct(true, "listlen".to_string(),"num".to_string()); 
    body_str.header_vars.push(
        // listlen(v) argumant : v (list)
        VarStruct::construct(
            "".to_string(),"v".to_string(),"list".to_string(),0,true,true
        )
    );
    // add to func list
    func_list.push(body_str);

    // exit(n) func
    body_str=  BodyStruc::construct(true, "exit".to_string(),"void".to_string()); 
    body_str.header_vars.push(
        // exit(n) argumant : n (list)
        VarStruct::construct(
            "".to_string(),"n".to_string(),"num".to_string(),0,true,true
        )
    );
    // add to func list
    func_list.push(body_str);

    //numread() func
    body_str=  BodyStruc::construct(true, "numread".to_string(),"num".to_string()); 
    // add to func list
    func_list.push(body_str);

    prog(&mut func_list);
}