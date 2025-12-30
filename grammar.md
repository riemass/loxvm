program -> declaration* EOF ;


declaration -> classDecl
            |  functionDecl
            |  varDecl
            |  statement ;

classDecl -> "class" ID ( "<" ID ) ?
             "{" function* "}" ; 

statement -> exprStmt
          |  forStmt
          |  ifStmt
          |  printStmt
          |  returnStmt
          |  whileStmt
          |  block ;

exprStmt -> expression ";" ;

