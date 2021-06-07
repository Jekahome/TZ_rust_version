
/// Нужно реализовать структуру данных с интерфейсом insert, remove, get (как у коллекции Map),
/// а также добавить поддержку версионирования (вспомни git) со следующим интерфейсом:
///
/// Checkpoint - сохранить текущую версию;
/// Rollback - откатить на определенную версию;
/// Prune - забыть все версии кроме последней.
//
/// Нельзя использовать сторонние библиотеки, только std, запись на диск не требуется
/// Рекомендованное время выполнения - не более 4х часов.

/// Там в задании не написано про тесты, но можно про них сказать,
/// что желательно их иметь в каком-то виде.
/// Плюс уделить вопрос многопоточности.
///
/// Но а так решение по желанию их вариантов несколько,
/// желательно самое оптимальное по их мнениею, и описание почему так приветствуется


use std::str::FromStr;
use std::cmp::PartialEq;

#[derive(Debug,PartialEq,Clone)]
struct Version{
    maje:String,
    minor:String,
    patch:String
}
impl Version{
    fn new( maje:String, minor:String,patch:String)->Self{
        Self{maje,minor,patch}
    }
}

#[derive(Debug)]
struct Storage{
    data:Vec<(Version,String)>
}

impl Storage{
    fn new()->Self{
        Self{data:vec![]}
    }
    ///  Сохранить текущую версию.
    fn checkpoint<K,V>(&mut self,version:Version,map:&Map<K,V>) where K: ToString,V: ToString {
        let mut buf:String = "".to_string();
        for (k,v) in map.data.iter(){
            buf.push_str(&k.to_string());
            buf.push_str(",");
            buf.push_str(&v.to_string());
            buf.push_str(";");
        }
        buf.pop();
        self.data.push((version,buf));
    }
    /// Откатить на определенную версию.
    fn rollback<K:PartialEq + FromStr,V: FromStr>(&self,version:Version,map:&mut Map<K,V>)->bool {
        if let Some(index) = self.data.iter().position(|(k,_)| k == &version){
            let (_,v) = self.data.get(index).unwrap();
            if let Ok(m) = Map::from_str(v){
                *map = m;
                return true;
            }
        }
        return false;
    }
    /// Забыть все версии кроме последней.
    fn prune(&mut self){
        if let Some(e) = self.data.pop(){
            self.data = vec![e];
        }
    }
}


#[derive(Debug)]
struct Map<K,V>{
    data:Vec<(K,V)>
}

impl <K: PartialEq + FromStr,V: FromStr> FromStr for Map<K,V> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m:Map<K,V> = Map::new();
        let coords: Vec<&str> = s.split(';').collect();
        for item in coords.iter(){
            let kv:Vec<&str> = item.split(",").collect();
            let k = match kv[0].parse::<K>() {
                Ok(k) => k,
                Err(_) =>{ return Err("Key parse error".into())}
            };
            let v = match kv[1].parse::<V>() {
                Ok(k) => k,
                Err(_) =>{ return Err("Value parse error".into())}
            };
            m.insert(k, v);
        }
        Ok(m)
    }
}

impl <K:PartialEq,V>Map<K,V> {
    fn new()->Self{
        Self{data:Vec::new()}
    }
    fn insert(&mut self,key:K,value:V)->(){
        if self.data.iter().find(|(k,_)| k == &key).is_none(){
            self.data.push((key,value));
        }else{
            for (k,v) in self.data.iter_mut() {
                if *k == key {
                    *v=value;
                    return;
                }
            }
        }
        return;
    }
    fn remove(&mut self,key:K){
        if let Some(index) = self.data.iter().position(|(k,_)| k == &key){
            self.data.remove(index);
        }
    }
    fn get(&mut self,key:K)->Option<&V>{
        if let Some(index) = self.data.iter().position(|(k,_)| k == &key){
            return match  self.data.get(index) {
                Some((_,v)) => Some(v),
                None =>  None
            }
        }
        return None;
    }
}


fn main(){

    let mut m:Map<String,i32> = Map::new();
    m.insert("1".into(),11);
    m.insert("2".into(),22);
    m.insert("3".into(),33);

    let mut storage = Storage::new();
    let ver1 = Version::new("1".into(),"1".into(),"1".into());
    storage.checkpoint(ver1.clone(),&m);

    m.remove("3".into());
    let ver2 = Version::new("1".into(),"1".into(),"2".into());
    storage.checkpoint(ver2.clone(),&m);

    println!("{:?}",&m);
    storage.rollback(ver1.clone(),&mut m);
    println!("{:?}",&m);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_insert() {
        let mut m:Map<String,i32> = Map::new();
        m.insert("1".into(),11);
        assert_eq!(m.get("1".into()), Some(&11));
    }

    #[test]
    fn test_map_remove() {
        let mut m:Map<String,i32> = Map::new();
        m.insert("1".into(),11);
        m.remove("1".into());
        assert_eq!(m.get("1".into()), None);
    }

    #[test]
    fn test_storage_checkpoint() {
        let mut m:Map<String,i32> = Map::new();
        m.insert("1".into(),11);
        m.insert("2".into(),22);
        m.insert("3".into(),33);

        let mut storage = Storage::new();
        let ver1 = Version::new("1".into(),"1".into(),"1".into());
        storage.checkpoint(ver1.clone(),&m);

        m.remove("1".into());
        m.remove("2".into());
        m.remove("3".into());

        assert_eq!(m.get("1".into()), None);
        assert_eq!(m.get("2".into()), None);
        assert_eq!(m.get("3".into()), None);

        storage.rollback(ver1.clone(),&mut m);

        assert_eq!(m.get("1".into()), Some(&11));
        assert_eq!(m.get("2".into()), Some(&22));
        assert_eq!(m.get("3".into()), Some(&33));
    }
    #[test]
    fn test_storage_prune() {
        let mut m: Map<String,i32> = Map::new();
        let mut storage = Storage::new();

        m.insert("1".into(), 11);
        let ver = Version::new("1".into(),"1".into(),"1".into());
        storage.checkpoint(ver,&m);

        m.insert("2".into(), 22);

        let ver = Version::new("1".into(),"1".into(),"2".into());
        storage.checkpoint(ver,&m);

        m.insert("3".into(), 33);
        let ver3 = Version::new("1".into(),"1".into(),"3".into());
        storage.checkpoint(ver3.clone(),&m);

        storage.prune();

        assert_eq!(1,storage.data.len());
        let (v,_) = &storage.data[0];
        assert_eq!(&ver3,v);
    }

}


