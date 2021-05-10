main = 
    -- putStrLn "What is your name"
    -- >> getLine
    -- >>= \name -> putStrLn ("hello, " ++ name ++ "!") 

    -- putStrLn "Enter a number"
    -- numStr <- getLine
    -- let num = (read numStr::Double)
    -- putStrLn (show (sqrt num))

    (putStrLn "Enter a number">> (putStrLn "12"))
    