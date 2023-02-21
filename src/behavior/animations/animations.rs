use serde::{Deserialize, Serialize};

#[doc = "Molang definition."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A(pub String);
impl std::ops::Deref for A {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<A> for String {
    fn from(value: A) -> Self {
        value.0
    }
}
impl From<&A> for A {
    fn from(value: &A) -> Self {
        value.clone()
    }
}
impl From<String> for A {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for A {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for A {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "A single animation definition for."]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Animation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation_length: Option<f64>,
    #[doc = "Whenever this animation should loop once it reaches the end, will only happen if the animation is still active."]
    #[serde(rename = "loop", default, skip_serializing_if = "Option::is_none")]
    pub loop_: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Timeline>,
}
impl From<&Animation> for Animation {
    fn from(value: &Animation) -> Self {
        value.clone()
    }
}
impl Animation {
    pub fn builder() -> builder::Animation {
        builder::Animation::default()
    }
}
#[doc = "A object specification on how to transition."]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationSpecification {
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, A>,
}
impl From<&AnimationSpecification> for AnimationSpecification {
    fn from(value: &AnimationSpecification) -> Self {
        value.clone()
    }
}
impl AnimationSpecification {
    pub fn builder() -> builder::AnimationSpecification {
        builder::AnimationSpecification::default()
    }
}
#[doc = "The animation specification."]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationsSchema {
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Animation>,
}
impl From<&AnimationsSchema> for AnimationsSchema {
    fn from(value: &AnimationsSchema) -> Self {
        value.clone()
    }
}
impl AnimationsSchema {
    pub fn builder() -> builder::AnimationsSchema {
        builder::AnimationsSchema::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Animationspec {
    Variant0(String),
    Variant1(AnimationSpecification),
}
impl From<&Animationspec> for Animationspec {
    fn from(value: &Animationspec) -> Self {
        value.clone()
    }
}
#[doc = "A version that tells minecraft what type of data format can be expected when reading this file."]
#[derive(Clone, Debug, Serialize)]
pub struct B(String);
impl std::ops::Deref for B {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<B> for String {
    fn from(value: B) -> Self {
        value.0
    }
}
impl From<&B> for B {
    fn from(value: &B) -> Self {
        value.clone()
    }
}
impl Default for B {
    fn default() -> Self {
        B("1.19.40".to_string())
    }
}
impl std::str::FromStr for B {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^([1-9]+)\\.([0-9]+)\\.([0-9]+)$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^([1-9]+)\\.([0-9]+)\\.([0-9]+)$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for B {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for B {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for B {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for B {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "The event or commands to execute."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Commands {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Variable>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<MinecraftCommand>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<Molang>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<Event>,
}
impl From<&Commands> for Commands {
    fn from(value: &Commands) -> Self {
        value.clone()
    }
}
impl Commands {
    pub fn builder() -> builder::Commands {
        builder::Commands::default()
    }
}
#[doc = "An event to be called upon within the executing entity."]
#[derive(Clone, Debug, Serialize)]
pub struct Event(String);
impl std::ops::Deref for Event {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Event> for String {
    fn from(value: Event) -> Self {
        value.0
    }
}
impl From<&Event> for Event {
    fn from(value: &Event) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Event {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^@s .*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^@s .*$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Event {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Event {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Event {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Executes a minecraft command."]
#[derive(Clone, Debug, Serialize)]
pub struct MinecraftCommand(String);
impl std::ops::Deref for MinecraftCommand {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<MinecraftCommand> for String {
    fn from(value: MinecraftCommand) -> Self {
        value.0
    }
}
impl From<&MinecraftCommand> for MinecraftCommand {
    fn from(value: &MinecraftCommand) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for MinecraftCommand {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^/[a-z].*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^/[a-z].*$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for MinecraftCommand {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MinecraftCommand {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MinecraftCommand {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for MinecraftCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Molang(String);
impl std::ops::Deref for Molang {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Molang> for String {
    fn from(value: Molang) -> Self {
        value.0
    }
}
impl From<&Molang> for Molang {
    fn from(value: &Molang) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Molang {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("[A-Za-z][a-z]*\\.[a-z_0-9]*")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"[A-Za-z][a-z]*\\.[a-z_0-9]*\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Molang {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Molang {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Molang {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Molang {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParticleEffectSpec {
    #[doc = "Set to false to have the effect spawned in the world without being bound to an actor (by default an effect is bound to the actor)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind_to_actor: Option<bool>,
    #[doc = "The name of a particle effect that should be played."]
    pub effect: String,
    #[doc = "The name of a locator on the actor where the effect should be located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    #[doc = "A molang script that will be run when the particle emitter is initialized."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_effect_script: Option<String>,
}
impl From<&ParticleEffectSpec> for ParticleEffectSpec {
    fn from(value: &ParticleEffectSpec) -> Self {
        value.clone()
    }
}
impl ParticleEffectSpec {
    pub fn builder() -> builder::ParticleEffectSpec {
        builder::ParticleEffectSpec::default()
    }
}
#[doc = "A timeline specification, property names are timestamps."]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Timeline {
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, TimelineExtraExtra>,
}
impl From<&Timeline> for Timeline {
    fn from(value: &Timeline) -> Self {
        value.clone()
    }
}
impl Timeline {
    pub fn builder() -> builder::Timeline {
        builder::Timeline::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TimelineExtraExtra {
    Variant0(Commands),
    Variant1(Vec<Commands>),
}
impl From<&TimelineExtraExtra> for TimelineExtraExtra {
    fn from(value: &TimelineExtraExtra) -> Self {
        value.clone()
    }
}
#[doc = "Sets the value to a molang variable."]
#[derive(Clone, Debug, Serialize)]
pub struct Variable(String);
impl std::ops::Deref for Variable {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Variable> for String {
    fn from(value: Variable) -> Self {
        value.0
    }
}
impl From<&Variable> for Variable {
    fn from(value: &Variable) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Variable {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^.*=.*;$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^.*=.*;$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Variable {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Variable {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Variable {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Variable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
mod builder {
    pub struct Animation {
        animation_length: Result<Option<f64>, String>,
        loop_: Result<Option<bool>, String>,
        timeline: Result<Option<super::Timeline>, String>,
    }
    impl Default for Animation {
        fn default() -> Self {
            Self {
                animation_length: Ok(Default::default()),
                loop_: Ok(Default::default()),
                timeline: Ok(Default::default()),
            }
        }
    }
    impl Animation {
        pub fn animation_length<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.animation_length = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for animation_length: {}",
                    e
                )
            });
            self
        }
        pub fn loop_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.loop_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for loop_: {}", e));
            self
        }
        pub fn timeline<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Timeline>>,
            T::Error: std::fmt::Display,
        {
            self.timeline = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeline: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Animation> for super::Animation {
        type Error = String;
        fn try_from(value: Animation) -> Result<Self, String> {
            Ok(Self {
                animation_length: value.animation_length?,
                loop_: value.loop_?,
                timeline: value.timeline?,
            })
        }
    }
    pub struct AnimationSpecification {
        extra: Result<std::collections::HashMap<String, super::A>, String>,
    }
    impl Default for AnimationSpecification {
        fn default() -> Self {
            Self {
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl AnimationSpecification {
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::A>>,
            T::Error: std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AnimationSpecification> for super::AnimationSpecification {
        type Error = String;
        fn try_from(value: AnimationSpecification) -> Result<Self, String> {
            Ok(Self {
                extra: value.extra?,
            })
        }
    }
    pub struct AnimationsSchema {
        extra: Result<std::collections::HashMap<String, super::Animation>, String>,
    }
    impl Default for AnimationsSchema {
        fn default() -> Self {
            Self {
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl AnimationsSchema {
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::Animation>>,
            T::Error: std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AnimationsSchema> for super::AnimationsSchema {
        type Error = String;
        fn try_from(value: AnimationsSchema) -> Result<Self, String> {
            Ok(Self {
                extra: value.extra?,
            })
        }
    }
    pub struct Commands {
        subtype_0: Result<Option<super::Variable>, String>,
        subtype_1: Result<Option<super::MinecraftCommand>, String>,
        subtype_2: Result<Option<super::Molang>, String>,
        subtype_3: Result<Option<super::Event>, String>,
    }
    impl Default for Commands {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
            }
        }
    }
    impl Commands {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Variable>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MinecraftCommand>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Molang>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Event>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Commands> for super::Commands {
        type Error = String;
        fn try_from(value: Commands) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
            })
        }
    }
    pub struct ParticleEffectSpec {
        bind_to_actor: Result<Option<bool>, String>,
        effect: Result<String, String>,
        locator: Result<Option<String>, String>,
        pre_effect_script: Result<Option<String>, String>,
    }
    impl Default for ParticleEffectSpec {
        fn default() -> Self {
            Self {
                bind_to_actor: Ok(Default::default()),
                effect: Err("no value supplied for effect".to_string()),
                locator: Ok(Default::default()),
                pre_effect_script: Ok(Default::default()),
            }
        }
    }
    impl ParticleEffectSpec {
        pub fn bind_to_actor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.bind_to_actor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bind_to_actor: {}", e));
            self
        }
        pub fn effect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.effect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect: {}", e));
            self
        }
        pub fn locator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.locator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for locator: {}", e));
            self
        }
        pub fn pre_effect_script<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.pre_effect_script = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for pre_effect_script: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<ParticleEffectSpec> for super::ParticleEffectSpec {
        type Error = String;
        fn try_from(value: ParticleEffectSpec) -> Result<Self, String> {
            Ok(Self {
                bind_to_actor: value.bind_to_actor?,
                effect: value.effect?,
                locator: value.locator?,
                pre_effect_script: value.pre_effect_script?,
            })
        }
    }
    pub struct Timeline {
        extra: Result<std::collections::HashMap<String, super::TimelineExtraExtra>, String>,
    }
    impl Default for Timeline {
        fn default() -> Self {
            Self {
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl Timeline {
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::TimelineExtraExtra>>,
            T::Error: std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Timeline> for super::Timeline {
        type Error = String;
        fn try_from(value: Timeline) -> Result<Self, String> {
            Ok(Self {
                extra: value.extra?,
            })
        }
    }
}
