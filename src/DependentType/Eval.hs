module DependentType.Eval where

import           DependentType.Core (Term (..))

data Value
    = VType
    | VVar Int
    | VApp Value Value
    | VPi String Value (Value -> Value)
    | VLam String Value (Value -> Value)
    | VSigma String Value (Value -> Value)
    | VPair Value Value
    | VFst Value
    | VSnd Value

newtype Env = Env [Value]

emptyEnv :: Env
emptyEnv = Env []

appendEnv :: Value -> Env -> Env
appendEnv v (Env env) = Env (v : env)

envAt :: Env -> Int -> Value
envAt (Env env) i = env !! i

eval :: Env -> Term -> Value
eval _ Type            = VType
eval env (Var i)       = envAt env i
eval env (Pi x a b)    = VPi x (eval env a) (\v -> eval (appendEnv v env) b)
eval env (Lam x a t)   = VLam x (eval env a) (\v -> eval (appendEnv v env) t)
eval env (App f x)     = vApp (eval env f) (eval env x)
eval env (Ann t _)     = eval env t
eval env (Sigma x a b) = VSigma x (eval env a) (\v -> eval (appendEnv v env) b)
eval env (Pair a b)    = VPair (eval env a) (eval env b)
eval env (Fst p)       = vFst (eval env p)
eval env (Snd p)       = vSnd (eval env p)

vApp :: Value -> Value -> Value
vApp (VLam _ _ f) v = f v
vApp t v            = VApp t v

vFst :: Value -> Value
vFst (VPair a _) = a
vFst p            = VFst p

vSnd :: Value -> Value
vSnd (VPair _ b) = b
vSnd p            = VSnd p

quote :: Int -> Value -> Term
quote _ VType          = Type
quote n (VVar i)       = Var (n - i - 1)
quote n (VApp f x)     = App (quote n f) (quote n x)
quote n (VPi x a b)    = Pi x (quote n a) (quote (n + 1) (b (VVar n)))
quote n (VLam x a t)   = Lam x (quote n a) (quote (n + 1) (t (VVar n)))
quote n (VSigma x a b) = Sigma x (quote n a) (quote (n + 1) (b (VVar n)))
quote n (VPair a b)    = Pair (quote n a) (quote n b)
quote n (VFst p)       = Fst (quote n p)
quote n (VSnd p)       = Snd (quote n p)

nf :: Term -> Term
nf = quote 0 . eval (Env [])
