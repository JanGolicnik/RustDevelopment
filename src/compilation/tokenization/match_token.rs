#[macro_export]
macro_rules! match_token {
    ($token:expr, $token_type:pat, $err:literal) => {
        let token__ = $token;
        if matches!(token__, $token_type) {
            token__
        } else {
            return Err(CompilationError::new($err));
        }
    };
    ($token:expr, $token_type:pat => $token_type_block:block, $err:literal) => {{
    let token__ = $token;
    match token__ {
        $token_type => $token_type_block,
        _ => return Err(CompilationError::new($err)),
    }
    }};
    ($token:expr, $err:literal, $( $token_type:pat => $token_type_block:block),*) => {{
        let token__ = $token;
        match token__ {
            $($token_type => $token_type_block,)*
            _ => return Err(CompilationError::new($err)),
        }
    }};
    ($token:expr, $( $token_type:pat => $token_type_block:block),*) => {{
        let token__ = $token;
        match token__ {
            $($token_type => $token_type_block,)*
        }
    }};
    ($token:expr, $token_type:pat => $token_type_block:block) => {{
        let token__ = $token;
        match token__ {
            $token_type => $token_type_block,
        }
    }};
}