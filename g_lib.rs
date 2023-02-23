#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
/*
    sqlite = "0.30.3"
    serde_json = "1.0.59"
    tokio = { version = "1.24.2", features = ["full"] }
    rand = "0.8.5"
    reqwest = { version = "0.11", features = ["blocking", "json"] }
*/


pub fn Get_uCountValueInVector<T>(Value: T, Vector: &Vec<T>) -> usize
where
    T: std::cmp::PartialEq,
{
    let mut uCount: usize = 0;
    for i in 0..Vector.len() {
        if Vector[i] == Value {
            uCount += 1;
        }
    }

    return uCount;
}
pub fn Get_ivDifferentRandomNum(iMin: i32, iMax: i32, iCount: usize) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();

    for i in iMin..iMax + 1 {
        arr.push(i);
    }

    let mut returnArr: Vec<i32> = Vec::new();

    for _ in 0..iCount {
        let iRandomIndex = Get_iRandomNum(0, (arr.len() - 1) as i32) as usize;
        returnArr.push(arr[iRandomIndex]);
        arr.remove(iRandomIndex);
    }

    return returnArr;
}

pub fn Get_KeyByValueInHashMap<T, V>(map: &std::collections::HashMap<T, V>, value: V) -> Option<T>
where
    V: std::cmp::PartialEq,
    T: Clone,
    V: Clone,
{
    let map1 = (*map).clone();
    for (key, val) in map1.iter() {
        if *val == value {
            return Some((*key).clone());
        }
    }
    None
}

pub fn Sort_HashmapByKey<K, V>(HashMap: &mut std::collections::HashMap<K, V>)
where
    K: std::clone::Clone,
    K: std::cmp::Eq,
    K: std::hash::Hash,
    K: std::cmp::Ord,
    V: std::clone::Clone,
{
    let mut sorted_keys: Vec<K> = HashMap.keys().cloned().collect();
    sorted_keys.sort();

    let mut NewHashMap: std::collections::HashMap<K, V> = std::collections::HashMap::new();
    for key in sorted_keys {
        let Value = HashMap.get(&key).unwrap().to_owned();
        NewHashMap.insert(key, Value);
    }
    *HashMap = NewHashMap;
}
pub struct RiotDevelop {
    sRiotDeveloperApiKey: String,
    sUserName: String,
}

impl RiotDevelop {
    pub fn new(sRiotDeveloperApiKey: &str) -> Result<RiotDevelop, String> {
        let sUrl = "https://kr.api.riotgames.com/lol/platform/v3/champion-rotations/".to_owned()
            + "?api_key="
            + sRiotDeveloperApiKey;
        let jRiotUserInfo = Get_jWebApiResponse(sUrl);
        match (&jRiotUserInfo)["maxNewPlayerLevel"] {
            serde_json::Value::Null => {
                //println!("Err Invalid Riot Api Key : Get a new one.");
                return Err("Invalid Riot Api Key. Get a new one.".to_owned());
            }

            _ => {
                return Ok(RiotDevelop {
                    sRiotDeveloperApiKey: sRiotDeveloperApiKey.to_owned(),
                    sUserName: String::new(),
                })
            }
        }
    }

    fn Get_sUrlHexBysRiotGameName(sName: &str) -> String {
        let sName = sName.to_owned();
        let bName = sName.as_bytes();
        let mut sReturn = String::new();
        for i in 0..bName.len() {
            let sHex = Get_sHexByiDecimal(bName[i] as i32);
            sReturn += &("%".to_owned() + &sHex);
        }
        sReturn = sReturn.to_uppercase();
        return sReturn;
    }

    pub fn Get_jRiotUserInfo(&mut self, sUserName: &str) -> serde_json::Value {
        self.sUserName = sUserName.to_owned();
        let sUrl = "https://kr.api.riotgames.com/lol/summoner/v4/summoners/by-name/".to_owned()
            + &Self::Get_sUrlHexBysRiotGameName(sUserName)
            + "?api_key="
            + &self.sRiotDeveloperApiKey;
        let sRiotUserInfo = Get_sWebApiResponse(sUrl);
        let jRiotUserInfo = Get_JsonValueByString(&sRiotUserInfo);
        return jRiotUserInfo;
    }

    fn Get_sRiotUserPuuidByJson(jRiotUserInfo: serde_json::Value) -> String {
        return jRiotUserInfo["puuid"].as_str().unwrap().to_owned();
    }

    pub fn Get_sRiotUserPuuid(&mut self, sUserName: &str) -> String {
        let jRiotUserINfo = self.Get_jRiotUserInfo(sUserName);
        let sRiotUserPuuid = Self::Get_sRiotUserPuuidByJson(jRiotUserINfo);
        return sRiotUserPuuid;
    }
    fn Get_sMatchesByPuuid(&self, sPuuid: &str) -> String {
        //https://asia.api.riotgames.com/lol/match/v5/matches/by-puuid/N_EvL5aQUNbsgYZeoPxZzTWbjI1jazrGkEACnKQob4TCXPNaBEy-pFnsOUmZavX4sKJSdLES4rTZJw/ids?start=0&count=20
        let sUrl = "https://asia.api.riotgames.com/lol/match/v5/matches/by-puuid/".to_owned()
            + sPuuid
            + "/ids"
            + "?api_key="
            + &self.sRiotDeveloperApiKey;
        let ListOfLolMatches = Get_sWebApiResponse(sUrl);
        return ListOfLolMatches;
    }

    pub fn Get_svMatchsPuuid(&mut self, sUserName: &str) -> Vec<String> {
        let sRiotUserPuuid = self.Get_sRiotUserPuuid(sUserName);
        let sListOfLolMatches = Self::Get_sMatchesByPuuid(&self, &sRiotUserPuuid);
        return SplitString(&sListOfLolMatches[2..&sListOfLolMatches.len() - 2], "\",\"");
    }

    pub fn Get_jMatchTimeline(&self, sMatchPuuid: &str) -> serde_json::Value {
        let sUrl = "https://asia.api.riotgames.com/lol/match/v5/matches/".to_owned()
            + sMatchPuuid
            + "/timeline"
            + "?api_key="
            + &self.sRiotDeveloperApiKey;
        let sMatchTimeline = Get_sWebApiResponse(sUrl);
        let jMatchTimeline = Get_JsonValueByString(&sMatchTimeline);
        return jMatchTimeline;
        //https://asia.api.riotgames.com/lol/match/v5/matches/KR_6365192970/timeline
    }

    fn Get_hUserRecog(
        jMatchTimeline: &serde_json::Value,
    ) -> std::collections::HashMap<usize, String> {
        let mut hUserRecog: std::collections::HashMap<usize, String> =
            std::collections::HashMap::new();
        let jParticipantsInfo = &jMatchTimeline["info"]["participants"];
        for i in 0..Get_uLenOfJsonValue(&jParticipantsInfo) {
            let uParticipantId =
                (&jParticipantsInfo)[i]["participantId"].as_u64().unwrap() as usize;
            let sUserPuuid = (&jParticipantsInfo)[i]["puuid"]
                .as_str()
                .unwrap()
                .to_owned();
            hUserRecog.insert(uParticipantId, sUserPuuid);
            Sort_HashmapByKey(&mut hUserRecog);
        }
        return hUserRecog;
    }

    pub fn Get_vLolKillLogs<B>(
        &self,
        jMatchTimeline: &serde_json::Value,
        mut b: B,
    ) -> Vec<LolKillLog>
    where
        B: FnMut(&LolKillLog) -> bool,
    {
        let _jWatching = &jMatchTimeline["info"]["frames"];
        let mut vKillLogs: Vec<LolKillLog> = Vec::new();
        let hUserRecog = Self::Get_hUserRecog(&jMatchTimeline);
        //(&_WatchingJson)[i]["events"][j]
        for i in 0..Get_uLenOfJsonValue(&_jWatching) {
            for j in 0..Get_uLenOfJsonValue(&_jWatching[i]["events"]) {
                //Get Victim Info
                let soVicName =
                    &_jWatching[i]["events"][j]["victimDamageDealt"][0]["name"].as_str();
                let sVicName = match soVicName {
                    None => continue,
                    Some(_) => soVicName.unwrap().to_owned(),
                };
                let uVictimNum =
                    *(&_jWatching[i]["events"][j]["victimId"].as_u64().unwrap()) as usize;

                let sVictimPuuid = hUserRecog.get(&uVictimNum).unwrap();
                let Victim = LolPlayer::new(&sVicName, uVictimNum, sVictimPuuid);

                let jVictimDamageReceived = &_jWatching[i]["events"][j]["victimDamageReceived"];

                //Get Killer Info
                let sKillerCharacter = jVictimDamageReceived
                    [Get_uLenOfJsonValue(jVictimDamageReceived) - 1]["name"]
                    .as_str()
                    .unwrap()
                    .to_owned();
                let uKillerNum =
                    (&_jWatching[i])["events"][j]["killerId"].as_u64().unwrap() as usize;

                let sKillerPuuid = hUserRecog.get(&uKillerNum).unwrap();

                let Killer = LolPlayer::new(&sKillerCharacter, uKillerNum, sKillerPuuid);

                //Get Relaters Info
                let mut vRelaters: Vec<LolPlayer> = Vec::new();
                for i in 0..Get_uLenOfJsonValue(jVictimDamageReceived) {
                    let sRelaterCharacter = (&jVictimDamageReceived)[i]["name"]
                        .as_str()
                        .unwrap()
                        .to_owned();
                    if sRelaterCharacter.contains("Minion") || sRelaterCharacter.contains("Turret")
                    {
                        continue;
                    }

                    let uRelaterNum =
                        *(&jVictimDamageReceived[i]["participantId"].as_u64().unwrap()) as usize;

                    // println!("sRelaterCharacter {}", sRelaterCharacter);
                    // println!("uRelaterNum {}", uRelaterNum);

                    // println!("hUserRecog : {:?}", &hUserRecog);
                    // println!("uRelaterNum : {}", &uRelaterNum);
                    let sRelaterPuuid = hUserRecog.get(&uRelaterNum).unwrap();
                    let Relater = LolPlayer::new(&sRelaterCharacter, uRelaterNum, sRelaterPuuid);
                    if vRelaters.contains(&Relater) {
                        continue;
                    }
                    vRelaters.push(Relater);
                }
                //sv_Relaters.retain(|s| !(*s).contains("Minion") && !(*s).contains("Turret"));

                //Get Time Info
                let uTimeStamp =
                    (&_jWatching)[i]["events"][j]["timestamp"].as_u64().unwrap() as usize / 1000;
                let uMin: usize = uTimeStamp / 60;
                let uSec: usize = uTimeStamp % 60;

                let KillLog = LolKillLog {
                    uMin: uMin,
                    uSec: uSec,
                    Victim: Victim,
                    Killer: Killer,
                    vRelaters: vRelaters,
                };
                //Test_LolKillLog {}

                vKillLogs.push(KillLog);
                //
                // uTimeStamp, uMin, uSec, sVicName, sKiller, v_Relaters
            }
        }
        vKillLogs.retain(|elem| b(elem));
        return vKillLogs;
    }
    pub fn Get_vAllLolKillLogs(&self, jMatchTimeline: &serde_json::Value) -> Vec<LolKillLog> {
        let vAllLolKillLogs = (&self).Get_vLolKillLogs(jMatchTimeline, |_| true);
        return vAllLolKillLogs;
    }

    pub fn Get_vLolKillLogsByUserName(
        &mut self,
        jMatchTimeline: &serde_json::Value,
        sUserName: &str,
    ) -> Vec<LolKillLog> {
        let sPlayerPuuid = self.Get_sRiotUserPuuid(sUserName);
        let vLolKillLogs = (&self).Get_vLolKillLogs(jMatchTimeline, |LolKillLog| {
            LolKillLog.Get_svRelatersPuuid().contains(&sPlayerPuuid)
        });
        return vLolKillLogs;
    }

    pub fn Get_sUserCharacter(&mut self, jMatchTimeline: &serde_json::Value, sUserName: &str) -> Option<String> {

        let sUserPuuid = self.Get_sRiotUserPuuid(sUserName);
        let _jWatching = &jMatchTimeline["info"]["frames"];
        let hUserRecog = Self::Get_hUserRecog(&jMatchTimeline);
        //(&_WatchingJson)[i]["events"][j]
        for i in 0..Get_uLenOfJsonValue(&_jWatching) {
            for j in 0..Get_uLenOfJsonValue(&_jWatching[i]["events"]) {
                //Get Victim Info
                let soVicName =
                    &_jWatching[i]["events"][j]["victimDamageDealt"][0]["name"].as_str();
                let sVicName = match soVicName {
                    None => continue,
                    Some(_) => soVicName.unwrap().to_owned(),
                };
                let uVictimNum =
                    *(&_jWatching[i]["events"][j]["victimId"].as_u64().unwrap()) as usize;

                let sVictimPuuid = hUserRecog.get(&uVictimNum).unwrap().to_owned();

                let jVictimDamageReceived = &_jWatching[i]["events"][j]["victimDamageReceived"];

                if sVictimPuuid == sUserPuuid {
                    return Some(sVicName);
                }
                //Get Killer Info
                let sKillerCharacter = jVictimDamageReceived
                    [Get_uLenOfJsonValue(jVictimDamageReceived) - 1]["name"]
                    .as_str()
                    .unwrap()
                    .to_owned();
                let uKillerNum =
                    (&_jWatching[i])["events"][j]["killerId"].as_u64().unwrap() as usize;

                let sKillerPuuid = hUserRecog.get(&uKillerNum).unwrap().to_owned();

                if sKillerPuuid == sUserPuuid {
                    return Some(sKillerCharacter);
                }
                //Get Relaters Info
                for i in 0..Get_uLenOfJsonValue(jVictimDamageReceived) {
                    let sRelaterCharacter = (&jVictimDamageReceived)[i]["name"]
                        .as_str()
                        .unwrap()
                        .to_owned();
                    if sRelaterCharacter.contains("Minion") || sRelaterCharacter.contains("Turret")
                    {
                        continue;
                    }

                    let uRelaterNum =
                        *(&jVictimDamageReceived[i]["participantId"].as_u64().unwrap()) as usize;

                    // println!("sRelaterCharacter {}", sRelaterCharacter);
                    // println!("uRelaterNum {}", uRelaterNum);

                    // println!("hUserRecog : {:?}", &hUserRecog);
                    // println!("uRelaterNum : {}", &uRelaterNum);
                    let sRelaterPuuid = hUserRecog.get(&uRelaterNum).unwrap().to_owned();
                    
                    if sRelaterPuuid == sUserPuuid {
                        return Some(sRelaterCharacter);
                    }
                }
                //sv_Relaters.retain(|s| !(*s).contains("Minion") && !(*s).contains("Turret"));

                //Get Time Info
                
                //
                // uTimeStamp, uMin, uSec, sVicName, sKiller, v_Relaters
            }
        }

        return None;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LolPlayer {
    pub sPlayerPuuid: String,
    pub uPlayerNum: usize,
    pub sPlayerCharacter: String,
}

impl LolPlayer {
    pub fn new(sPlayerCharacter: &String, uPlayerNum: usize, sPlayerPuuid: &String) -> LolPlayer {
        return LolPlayer {
            sPlayerPuuid: sPlayerPuuid.to_owned(),
            uPlayerNum: uPlayerNum,
            sPlayerCharacter: sPlayerCharacter.to_owned(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct LolKillLog {
    pub uMin: usize,
    pub uSec: usize,
    pub Victim: LolPlayer,
    pub Killer: LolPlayer,
    pub vRelaters: Vec<LolPlayer>,
}

impl LolKillLog {
    pub fn new(
        uMin: usize,
        uSec: usize,
        Victim: LolPlayer,
        Killer: LolPlayer,
        vRelaters: Vec<LolPlayer>,
    ) -> LolKillLog {
        return LolKillLog {
            uMin,
            uSec,
            Victim,
            Killer,
            vRelaters,
        };
    }

    pub fn Get_uTotalSeconds(&self) -> usize {
        let uTotalSeconds = (&self.uMin) * 60 + &self.uSec;
        return uTotalSeconds;
    }
    pub fn Get_bContainsCharacterName(&self, sCharacter: &str) -> bool {
        let svRelatersCharacter = Self::Get_svRelatersCharacter(&self);

        let bContainsCharacterName = svRelatersCharacter.contains(&sCharacter.to_owned());
        return bContainsCharacterName;
    }

    pub fn Get_svRelatersCharacter(&self) -> Vec<String> {
        let vRelaters = &self.vRelaters;
        let mut svRelaters: Vec<String> = Vec::new();
        for i in 0..vRelaters.len() {
            let Relater = &vRelaters[i];
            svRelaters.push((&Relater).sPlayerCharacter.to_owned());
        }

        return svRelaters;
    }

    pub fn Get_svRelatersPuuid(&self) -> Vec<String> {
        let vRelaters = &self.vRelaters;
        let mut svRelatersPuuid: Vec<String> = Vec::new();
        for i in 0..vRelaters.len() {
            let Relater = &vRelaters[i];
            svRelatersPuuid.push((&Relater).sPlayerPuuid.to_owned());
        }

        return svRelatersPuuid;
    }
}
impl PartialEq for LolKillLog {
    fn eq(&self, other: &Self) -> bool {
        self.Get_uTotalSeconds() == other.Get_uTotalSeconds()
    }
}
impl Eq for LolKillLog {}
impl Ord for LolKillLog {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.Get_uTotalSeconds().cmp(&other.Get_uTotalSeconds())
    }
}

impl PartialOrd for LolKillLog {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for LolKillLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sRelaters: Vec<String> = Vec::new();
        for i in 0..self.vRelaters.len() {
            let sRelater = &self.vRelaters[i].sPlayerCharacter;
            sRelaters.push(sRelater.to_owned());
        }
        return write!(
            f,
            "{}m {}s {} killed {} ( Relaters = {:?} )",
            self.uMin,
            self.uSec,
            self.Killer.sPlayerCharacter,
            self.Victim.sPlayerCharacter,
            sRelaters,
        );
    }
}
pub fn Get_sWebApiResponse(sUrl: String) -> String {
    let response = reqwest::blocking::get(sUrl).unwrap();
    return response.text().unwrap();
}

pub fn Get_jWebApiResponse(sUrl: String) -> serde_json::Value {
    let response = reqwest::blocking::get(sUrl).unwrap();
    let sResponse = response.text().unwrap();
    let jResponse = Get_JsonValueByString(&sResponse);
    return jResponse;
}

pub fn Get_sPointerAddress<T>(variable: &T) -> *const T {
    let a = variable as *const _;
    return a;
}

pub fn Get_sHexByiDecimal(iDecimal: i32) -> String {
    return format!("{:x}", iDecimal);
}

pub fn EncodeUtf8(s_: String) -> Vec<u8> {
    return s_.as_bytes().to_owned();
}

pub fn std_flush() {
    std::io::Write::flush(&mut std::io::stdout().lock())
        .expect("Error g_lib line 16: Failed to flush stdout");
}
#[inline]
pub fn WaitUntilEnter() {
    std_flush();
    let _ = Get_sInput("Wait Until Inputed :");
}

#[inline]
pub fn Get_sCurrentDirectory() -> String {
    return std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
}

#[inline]
pub fn Get_Db(s_FilePartName: &str) -> Db {
    return Db {
        ConnToDb: sqlite::open(s_FilePartName)
            .expect("Error g_lib line 33: Failed to create sqlite"),
    };
}
pub struct Db {
    ConnToDb: sqlite::Connection,
}
impl Db {
    pub fn new(s_FilePartName: &str) -> Db {
        return Db {
            ConnToDb: sqlite::open(s_FilePartName)
                .expect("Error g_lib line 33: Failed to create sqlite"),
        };
    }
    pub fn create_table(&self, sTable: &str, sValues: &str) {
        //CREATE TABLE kr_KR ( s_icd_code TEXT, s_en_name TEXT, s_local_name TEXT)
        let _ =
            &self.execute_sQuery(&("CREATE TABLE ".to_string() + sTable + " (" + sValues + ")"));
    }

    pub fn insert(&self, sTable: &str, sValues: &str) {
        //INSERT INTO kr_KR ("A00.0" , "Cholera due to Vibrio cholerae 01, biovar cholerae", "鍮꾨툕由ъ삤 肄쒕젅?씪 01 肄쒕젅?씪?삎洹좎뿉 ?쓽?븳 肄쒕젅?씪")
        let _ = &self
            .execute_sQuery(&("INSERT INTO ".to_string() + sTable + " VALUES (" + sValues + ")"));
    }

    pub fn update(&self, sTable: &str, sSetVariables: &str, sCondition: &str) {
        //UPDATE kr_KR SET s_en_name = "Cholera due to Vibrio cholerae 01, biovar cholerae", s_local_name = "鍮꾨툕由ъ삤 肄쒕젅?씪 01 肄쒕젅?씪?삎洹좎뿉 ?쓽?븳 肄쒕젅?씪" WHERE s_icd_code="A00.0"
        let _ = &self.execute_sQuery(
            &("UPDATE ".to_string()
                + &sTable
                + " SET "
                + sSetVariables
                + " WHERE "
                + &sCondition.replace("==", "=")),
        );
    }

    pub fn select<'a>(
        &'a self,
        sTable: &str,
        sCondition: &str,
    ) -> impl Iterator<Item = sqlite::Row> + 'a {
        // sCondition1 = r#"s_en_name LIKE '%Cholera%'"#;
        // sCondition2 = r#"s_en_name = "Cholera due to Vibrio cholerae 01, biovar cholerae""#;

        //SELECT * FROM kr_KR WHERE s_en_name LIKE '%due%';

        let sQuery: String =
            "SELECT * FROM ".to_string() + &sTable + " WHERE " + &sCondition.replace("==", "=");
        let g_rows = (&self)
            .ConnToDb
            .prepare(sQuery)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        //let b = &a.cloned();
        return g_rows;
    }
    // let query = "SELECT * FROM kr_KR";
    // let connection = sqlite::open("memory.sqlite3").unwrap();

    // for row in connection
    //     .prepare(query)
    //     .unwrap()
    //     .into_iter()
    //     .map(|row| row.unwrap())
    // {

    pub fn delete(&self, sTable: &str, sCondition: &str) {
        let _ = &self.execute_sQuery(
            &("DELETE FROM ".to_string() + sTable + " WHERE " + &sCondition.replace("==", "=")),
        );
    }

    pub fn execute_sQuery(&self, sQuery: &str) {
        let _ = &self
            .ConnToDb
            .execute(sQuery)
            .expect("Error g_lib line 64: Given Command might be wrong. Check it Again");
    }
}

#[inline]
pub fn Get_sValueFromDbRow(sColumn: &str, row: sqlite::Row) -> String {
    let a = row.read::<&str, _>(sColumn).to_owned();
    return a;
}
#[inline]
pub fn Get_sTypeOfVariable<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}

#[inline]
pub fn Get_JsonValueByString(sText: &str) -> serde_json::Value {
    let JsonValue: serde_json::Value =
        { serde_json::from_str::<serde_json::Value>(sText).unwrap() };
    // Save the JSON structure into the other file.
    return JsonValue;
}

#[inline]
pub fn Get_sTextByFileName(sFileName: String) -> String {
    let text = std::fs::read_to_string(&sFileName).unwrap();
    return text;
}

#[inline]
pub fn SaveJsonByJsonValue(sJsonValue: serde_json::Value, sFileName: String) {
    std::fs::write(
        sFileName,
        serde_json::to_string_pretty(&sJsonValue).unwrap(),
    )
    .unwrap();
}

#[inline]
pub fn SaveJsonByString(sFileName: String, sText: &str) {
    let _JsonValue = Get_JsonValueByString(sText);
    SaveJsonByJsonValue(_JsonValue, sFileName);
}

#[inline]
pub fn Decode_utf8(data: &[u8]) -> &str {
    return std::str::from_utf8(&data).expect("Error g_lib line 127: Failed to Decode_utf8");
}

pub struct TcpServer {
    listener: std::net::TcpListener,
}

impl TcpServer {
    pub fn new(sIp: &str, iPort: u16) -> TcpServer {
        let sAddress: String = sIp.to_owned() + ":" + &iPort.to_string();

        let sErrorMsg =
            "Error g_lib line 135 :Failed to Create TcpListnerer at ".to_owned() + &sAddress;
        let listener = std::net::TcpListener::bind(&sAddress).expect(&sErrorMsg);
        println!("Server listening on [{}]", sAddress);
        let TcpServer = TcpServer { listener: listener };
        return TcpServer;
    }
    pub fn start_s2s(&self, func: fn(String) -> String) {
        for stream in (&self).listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "New connection : [{}]",
                        stream.peer_addr().unwrap().to_string()
                    );
                    std::thread::spawn(move || {
                        // connection succeeded
                        Self::handle_client(stream, func)
                    });
                }

                Err(e) => {
                    println!("Error line 158: {}", e);
                    /* connection failed */
                }
            }
        }
    }

    fn handle_client(mut stream: std::net::TcpStream, func: fn(String) -> String) {
        const iByteSize: usize = 4096;

        let mut byteFromClient: &[u8];
        let u8EndUtf8Code = &3;
        loop {
            let mut byte = [0 as u8; iByteSize]; // using 5 byte buffer

            match std::io::Read::read(&mut stream, &mut byte) {
                Ok(size) => {
                    byteFromClient = &byte[0..size];
                    let mut sClientMsg: String = String::new();
                    sClientMsg.push_str(Decode_utf8(byteFromClient));

                    if byteFromClient.contains(u8EndUtf8Code) {
                        //Main
                        let sReturnText: String = func(sClientMsg.clone());

                        let sClientAddr = stream.peer_addr().unwrap().to_string();

                        match std::io::Write::write(&mut stream, sReturnText.as_bytes()) {
                            Ok(_) => {
                                println!(
                                    "[{}] sClientMsg :{}, sReturnText :{};",
                                    sClientAddr, sClientMsg, sReturnText
                                );
                            }

                            Err(_) => {
                                println!(
                                        "An error occurred while sending Msg, terminating connection with {}",
                                        stream.peer_addr().unwrap()
                                    );
                                stream.shutdown(std::net::Shutdown::Both).unwrap();
                                break;
                            }
                        }
                    }
                }

                Err(_) => {
                    println!(
                        "An error occurred, terminating connection with {}",
                        stream.peer_addr().unwrap()
                    );
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                    return;
                }
            }
        }
    }
}

impl Drop for TcpServer {
    fn drop(&mut self) {
        let sAddress = &self.listener.local_addr().unwrap().to_string();
        drop(&self.listener);
        println!("g_lib::TcpServer at {} closed.", sAddress);
    }
}

#[inline]
pub fn Get_bFileInDirectory(sName: &str) -> bool {
    let path = std::path::Path::new(sName);
    return path.exists();
}

#[inline]
pub fn Get_sIpNiPort() -> (String, u16) {
    let sGoogleIp = "142.251.42.132:80";
    let _TcpStream = std::net::TcpStream::connect(sGoogleIp).unwrap();
    let local_addr = _TcpStream.local_addr().unwrap();
    return (local_addr.ip().to_string(), local_addr.port());
}


#[inline]
pub fn sleep(fSeconds: f32) {
    std::thread::sleep(std::time::Duration::from_secs_f32(fSeconds));
}

#[inline]
pub fn Get_iRandomNum(iMin: i32, iMax: i32) -> i32 {
    assert_eq!(iMin <= iMax, true);
    let mut rng = rand::thread_rng();
    let random_number = rand::Rng::gen_range(&mut rng, iMin..iMax + 1);
    return random_number;
}

#[inline]
pub fn Get_sInput(sInputText: &str) -> String {
    print!("{}", sInputText);

    let mut sInput = String::new();
    std::io::stdin().read_line(&mut sInput).unwrap();
    sInput = sInput[..sInput.len() - 2].to_owned();

    return sInput;
}
pub fn Get_uLenOfJsonValue(js_: &serde_json::Value) -> usize {
    let uLenOfJson = match js_ {
        serde_json::Value::Object(map) => map.len(),
        serde_json::Value::Array(vec) => vec.len(),
        _ => 0, // for non-object and non-array values
    };

    return uLenOfJson;
}

#[inline]
pub fn SplitString<T>(sText: &str, sSplitText: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    return sText
        .split(sSplitText)
        .map(|s| s.parse().unwrap())
        .collect();
}

#[inline]
pub fn RemoveValueFromVec<T>(value: T, vec: &mut Vec<T>)
where
    T: std::cmp::PartialEq,
{
    let mut i: usize = 0;

    while i < vec.len() {
        if vec[i] == value {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
