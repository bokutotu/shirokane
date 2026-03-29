module DependentType.Eval where

import           DependentType.Core (Term (..))

data Value
    = VType
    | VVar Int
    | VApp Value Value
    | VPi String Value (Value -> Value)
    | VLam String Value (Value -> Value)

newtype Env = Env [Value]

emptyEnv :: Env
emptyEnv = Env []

appendEnv :: Value -> Env -> Env
appendEnv v (Env env) = Env (v : env)

envAt :: Env -> Int -> Value
envAt (Env env) i = env !! i

eval :: Env -> Term -> Value
eval _ Type          = VType
eval env (Var i)     = envAt env i
eval env (Pi x a b)  = VPi x (eval env a) (\v -> eval (appendEnv v env) b)
eval env (Lam x a t) = VLam x (eval env a) (\v -> eval (appendEnv v env) t)
eval env (App f x)   = vApp (eval env f) (eval env x)
eval env (Ann t _)   = eval env t

vApp :: Value -> Value -> Value
vApp (VLam _ _ f) v = f v
vApp t v            = VApp t v

quote :: Int -> Value -> Term
quote _ VType        = Type
quote n (VVar i)     = Var (n - i - 1)
quote n (VApp f x)   = App (quote n f) (quote n x)
quote n (VPi x a b)  = Pi x (quote n a) (quote (n + 1) (b (VVar n)))
quote n (VLam x a t) = Lam x (quote n a) (quote (n + 1) (t (VVar n)))

nf :: Term -> Term
nf = quote 0 . eval (Env [])
