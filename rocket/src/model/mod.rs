pub(crate) mod user;

use sqlx::error::Error::*;

pub fn sqlx_error<'a>(err: sqlx::error::Error) -> &'a str {
    match err {
        //연결 문자열을 구문 분석하는 동안 오류가 발생했습니다.
        Configuration(_) => todo!(),
        //데이터베이스에서 오류가 반환되었습니다.
        Database(_) => todo!(),
        //데이터베이스 백엔드와 통신하는 동안 오류가 발생했습니다.
        Io(_) => todo!(),
        //TLS 연결을 설정하는 동안 오류가 발생했습니다.
        Tls(_) => todo!(),
        //데이터베이스와 통신하는 동안 예기치 않거나 잘못된 데이터가 발생했습니다.
        //이것은 SQLx 드라이버에 프로그래밍 오류가 있거나 데이터베이스 자체에 대한 연결이 손상되었음을 나타냅니다.
        Protocol(_) => todo!(),
        //하나 이상의 행을 반환할 것으로 예상되는 쿼리에서 반환된 행이 없습니다.
        RowNotFound => "하나 이상의 행을 반환할 것으로 예상되는 쿼리에서 반환된 행이 없습니다.",//todo!(),
        //검색어가 존재하지 않습니다. 오타 또는 누락된 사용자 유형 때문일 수 있습니다.
        TypeNotFound { type_name: _ } => todo!(),
        //열 인덱스가 범위를 벗어났습니다.
        ColumnIndexOutOfBounds { index: _, len: _ } => todo!(),
        //지정된 이름에 대한 열을 찾을 수 없습니다.
        ColumnNotFound(_) => todo!(),
        //특정 열의 값을 디코딩하는 동안 오류가 발생했습니다.
        ColumnDecode {
            index: _,
            source: _,
        } => todo!(),
        //값을 디코딩하는 동안 오류가 발생했습니다.
        Decode(_) => todo!(),
        //Pool::acquire연결을 사용할 수 없거나 새 연결을 열려고 시도하는 동안 다른 작업에 너무 많은 오류가 발생하여 A 시간이 초과되었습니다.
        PoolTimedOut => todo!(),
        //Pool::close우리가 기다리는 동안 호출되었습니다 Pool::acquire.
        PoolClosed => todo!(),
        //백그라운드 작업자가 충돌했습니다.
        WorkerCrashed => todo!(),
        //
        Migrate(_) => todo!(),
        _ => "Unknown Error",
    }
}
