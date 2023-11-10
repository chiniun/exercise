

enum reqMethod {
    Get,
    Post
}

struct req {
    method: reqMethod,
    body: string,
}

struct resp {
    body: string,
}

fn req_do(r: &req)->resp{
    
}

