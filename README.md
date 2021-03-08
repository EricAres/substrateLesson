# substrate 2期Team6学号99 -Eric：李彦龙的作业
 

1. 第一课作业 ：存证以及长度限制

---

- git地址：https://github.com/EricAres/substrate-node-eric.git
- 之后迁移到：https://github.com/EricAres/substrateLesson
- 存证部分，创建、撤销、转移，采用的是substrate2.0

```
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
		pub fn transfer_claim(origin, claim: Vec<u8>, receiver: T::AccountId) -> dispatch::DispatchResult {

```
- 创建存证时长度限制：
Trait：

```
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	// setMax Length limit
	type MaxClaimLength: frame_support::traits::Get<u32>;
}
```
- Error&Event：

```
// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		LengthLimitOut,
	}
}
```
- runtime部分
runtime传入大小控制调用：修改3个地方
1>pub se pallet_poe;
2>实现,定义参数，并赋值

```
parameter_types! {
	pub const MaxClaimLength: u32 = 6;
}
// Configure FRAME pallets to include in runtime.
impl pallet_poe::Trait for Runtime{
	type Event =Event;
	type MaxClaimLength =MaxClaimLength;
}
```
- 注册construct_runtime!(
```
 PoeModule: pallet_poe::{Module, Call, Storage, Event<T>},
 KittiesModule: pallet_kitties::{Module, Call, Storage, Event<T>},
```

- 配置入口runtime/cargo.toml,增加如下节点的内容

```
[dependencies]
pallet-kitties = { path = '../pallets/kitties', default-features = false, version = '2.0.0' }
pallet-poe = { path = '../pallets/poe', default-features = false , version = '2.0.0' }
[features]
std = [
'pallet-poe/std',
    'pallet-kitties/std',
]
```
- ### 总结

```
第一课作业思考：
长度限制，可以用mem::size_of,用framesupport入口
长度限制：取定义本身的长度做判断值，而定义本身如果要动态，则通过runtime传入
断言用好：valid各种条件，不满足异常
测试：assert_noop!(和assert_noop!（！实现验证的ok、error双向
```

# 2. 第二课作业:Kitties

| 类别     | 内容                                  |
| -------- | ------------------------------------- |
| metadata | Storage/Events/Calls/Constants/Errors |
| assets   | owner/trans/breed                     |
| balances | trans...                              |



metadata、assets、balances
此课首次完成在
# 3. 第三课作业
ink contract练习
- 第一个例子，可以做到。用canvas
- 第二个例子，需要用到pallet调用，而pallet调用需要chainextention，这个时候需要升级substrate到3.0
- 升级substrate3.0过程

| 升级关键字                         | 升级内容                                         |
| ---------------------------------- | ------------------------------------------------ |
| kitties/poe两个模块迁移，替换Trait | 替换成Config                                     |
| node/chain-spec                    | 增加：ContractsConfig及实现和测试内容            |
| runtime                            | 增加pallet-contract内容，从substrate迁移，共12处 |
| node/cargo.toml                    | 增加pallet-contracts-rpc = '3.0.0'               |
| node/rpc.rs                        | pallet_contracts_rpc&&io.extend_with             |
```
增加use pallet_contracts_rpc::{Contracts, ContractsApi};
和io.extend_with(ContractsApi::to_delegate(Contracts::new(client.clone()));
```




4. 第四课作业
- 升级系统到substrat 3.0
- 打通智能合约和pallet通信
- 合约:erc20/flipper/incrementer/rand-extension
- 