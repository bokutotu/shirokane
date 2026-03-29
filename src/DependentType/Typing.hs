{-# LANGUAGE OverloadedRecordDot #-}

module DependentType.Typing where

import           DependentType.Core (Term (..))
import           DependentType.Eval (Env, Value (..), appendEnv, emptyEnv, eval,
                                     quote)

data Ctx = Ctx
    { env   :: Env
    , types :: [Value]
    , depth :: Int
    }

emptyCtx :: Ctx
emptyCtx = Ctx emptyEnv [] 0

bind :: Value -> Ctx -> Ctx
bind ty (Ctx env types depth) = Ctx (appendEnv ty env) (ty : types) (depth + 1)

conv :: Int -> Value -> Value -> Bool
conv n a b = quote n a == quote n b

infer :: Ctx -> Term -> Either String Value
infer _ Type = Right VType
infer ctx (Var i) = Right $ ctx.types !! i
infer ctx (Pi _ a b) = do
    check ctx a VType
    check (bind (eval ctx.env a) ctx) b VType
    Right VType
infer ctx (App f x) = do
    fTy <- infer ctx f
    case fTy of
        VPi _ a b -> do
            check ctx x a
            Right $ b (eval ctx.env x)
        _ -> Left "Expected a function type"
infer ctx (Ann t ty) = do
    check ctx ty VType
    let tyVal = eval ctx.env ty
    check ctx t tyVal
    Right tyVal
infer _ Lam{} = Left "Cannot infer type of lambda without annotation"

check :: Ctx -> Term -> Value -> Either String ()
check ctx (Lam _ _ t) (VPi _ a b) = check (bind a ctx) t (b (VVar ctx.depth))
check ctx v ty = do
    vTy <- infer ctx v
    if conv ctx.depth vTy ty
        then Right ()
        else Left $ "Type mismatch: expected " ++ show (quote ctx.depth ty) ++ ", got " ++ show (quote ctx.depth vTy)
