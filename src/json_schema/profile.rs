use super::{endorsement, general, identity};
use serde::{Deserialize, Serialize};

#[doc = "A Profile is a collection of information that describes the entity or organization using Open Badges. Issuers must be represented as Profiles, and endorsers, or other entities may also be represented using this vocabulary. Each Profile that represents an Issuer may be referenced in many BadgeClasses that it has defined. Anyone can create and host an Issuer file to start issuing Open Badges. Issuers may also serve as recipients of Open Badges, often identified within an Assertion by specific properties, like their url or contact email address."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Profile {
    #[doc = "Unique URI for the Issuer/Profile file."]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ProfileType,
    #[doc = "The name of the entity or organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The homepage or social media profile of the entity, whether individual or institutional. Should be a URL/URI Accessible via HTTP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "A phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "A short description of the issuer entity or organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement: Vec<endorsement::EndorsementCredential>,
    #[serde(rename = "endorsementJwt", default, skip_serializing_if = "Vec::is_empty")]
    pub endorsement_jwt: Vec<ProfileEndorsementJwtItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<general::Image>,
    #[doc = "An email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "otherIdentifier", default, skip_serializing_if = "Vec::is_empty")]
    pub other_identifier: Vec<identity::IdentifierEntry>,
    #[doc = "If the entity is an organization, `official` is the name of an authorized official of the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub official: Option<String>,
    #[serde(rename = "parentOrg", default, skip_serializing_if = "Option::is_none")]
    pub parent_org: Box<Option<Profile>>,
    #[doc = "Family name. In the western world, often referred to as the 'last name' of a person."]
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[doc = "Given name. In the western world, often referred to as the 'first name' of a person."]
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "Additional name. Includes what is often referred to as 'middle name' in the western world."]
    #[serde(rename = "additionalName", default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
    #[doc = "Patronymic name."]
    #[serde(rename = "patronymicName", default, skip_serializing_if = "Option::is_none")]
    pub patronymic_name: Option<String>,
    #[doc = "Honorific prefix(es) preceding a person's name (e.g. 'Dr', 'Mrs' or 'Mr')."]
    #[serde(rename = "honorificPrefix", default, skip_serializing_if = "Option::is_none")]
    pub honorific_prefix: Option<String>,
    #[doc = "Honorific suffix(es) following a person's name (e.g. 'M.D, PhD')."]
    #[serde(rename = "honorificSuffix", default, skip_serializing_if = "Option::is_none")]
    pub honorific_suffix: Option<String>,
    #[doc = "Family name prefix. As used in some locales, this is the leading part of a family name (e.g. 'de' in the name 'de Boer')."]
    #[serde(rename = "familyNamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub family_name_prefix: Option<String>,
    #[doc = "Birthdate of the person."]
    #[serde(rename = "dateOfBirth", default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::naive::NaiveDate>,
}
impl From<&Profile> for Profile {
    fn from(value: &Profile) -> Self {
        value.clone()
    }
}

#[doc = "Allows endorsers to make specific claims about the individual or organization represented by this profile. These endorsements are signed with the VC-JWT proof format."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ProfileEndorsementJwtItem(String);
impl std::ops::Deref for ProfileEndorsementJwtItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ProfileEndorsementJwtItem> for String {
    fn from(value: ProfileEndorsementJwtItem) -> Self {
        value.0
    }
}
impl From<&ProfileEndorsementJwtItem> for ProfileEndorsementJwtItem {
    fn from(value: &ProfileEndorsementJwtItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ProfileEndorsementJwtItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[a-zA-Z0-9_-]+\\.[a-zA-Z0-9_-]*\\.[a-zA-Z0-9_-]+$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ProfileEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ProfileEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ProfileEndorsementJwtItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ProfileEndorsementJwtItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ProfileType {
    String(String),
    VecString(Vec<String>),
}
impl From<&ProfileType> for ProfileType {
    fn from(value: &ProfileType) -> Self {
        value.clone()
    }
}
impl From<String> for ProfileType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for ProfileType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for ProfileType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for ProfileType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[doc = "An address for the described entity."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Address {
    #[serde(rename = "type")]
    pub type_: AddressType,
    #[doc = "A country."]
    #[serde(rename = "addressCountry", default, skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
    #[doc = "A country code. The value must be a ISO 3166-1 alpha-2 country code [[ISO3166-1]]."]
    #[serde(rename = "addressCountryCode", default, skip_serializing_if = "Option::is_none")]
    pub address_country_code: Option<String>,
    #[doc = "A region within the country."]
    #[serde(rename = "addressRegion", default, skip_serializing_if = "Option::is_none")]
    pub address_region: Option<String>,
    #[doc = "A locality within the region."]
    #[serde(rename = "addressLocality", default, skip_serializing_if = "Option::is_none")]
    pub address_locality: Option<String>,
    #[doc = "A street address within the locality."]
    #[serde(rename = "streetAddress", default, skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[doc = "A post office box number for PO box addresses."]
    #[serde(rename = "postOfficeBoxNumber", default, skip_serializing_if = "Option::is_none")]
    pub post_office_box_number: Option<String>,
    #[doc = "A postal code."]
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo: Option<GeoCoordinates>,
}
impl From<&Address> for Address {
    fn from(value: &Address) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AddressType {
    String(String),
    VecString(Vec<String>),
}
impl From<&AddressType> for AddressType {
    fn from(value: &AddressType) -> Self {
        value.clone()
    }
}
impl From<String> for AddressType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for AddressType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<Vec<String>> for AddressType {
    fn from(value: Vec<String>) -> Self {
        Self::VecString(value)
    }
}
impl From<Vec<&str>> for AddressType {
    fn from(value: Vec<&str>) -> Self {
        let v = value.iter().map(|v| v.to_string()).collect();
        Self::VecString(v)
    }
}

#[doc = "The geographic coordinates of a location."]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct GeoCoordinates {
    #[doc = "The value of the type property MUST be an unordered set. One of the items MUST be the IRI 'GeoCoordinates'."]
    #[serde(rename = "type")]
    pub type_: String,
    pub latitude: f64,
    pub longitude: f64,
}
impl From<&GeoCoordinates> for GeoCoordinates {
    fn from(value: &GeoCoordinates) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfileBuilder {
    additional_name: Result<Option<String>, String>,
    address: Result<Option<Address>, String>,
    date_of_birth: Result<Option<chrono::NaiveDate>, String>,
    description: Result<Option<String>, String>,
    email: Result<Option<String>, String>,
    endorsement: Result<Vec<endorsement::EndorsementCredential>, String>,
    endorsement_jwt: Result<Vec<ProfileEndorsementJwtItem>, String>,
    family_name: Result<Option<String>, String>,
    family_name_prefix: Result<Option<String>, String>,
    given_name: Result<Option<String>, String>,
    honorific_prefix: Result<Option<String>, String>,
    honorific_suffix: Result<Option<String>, String>,
    id: Result<String, String>,
    image: Result<Option<general::Image>, String>,
    name: Result<Option<String>, String>,
    official: Result<Option<String>, String>,
    other_identifier: Result<Vec<identity::IdentifierEntry>, String>,
    parent_org: Result<Box<Option<Profile>>, String>,
    patronymic_name: Result<Option<String>, String>,
    phone: Result<Option<String>, String>,
    type_: Result<ProfileType, String>,
    url: Result<Option<String>, String>,
}
impl Default for ProfileBuilder {
    fn default() -> Self {
        Self {
            additional_name: Ok(Default::default()),
            address: Ok(Default::default()),
            date_of_birth: Ok(Default::default()),
            description: Ok(Default::default()),
            email: Ok(Default::default()),
            endorsement: Ok(Default::default()),
            endorsement_jwt: Ok(Default::default()),
            family_name: Ok(Default::default()),
            family_name_prefix: Ok(Default::default()),
            given_name: Ok(Default::default()),
            honorific_prefix: Ok(Default::default()),
            honorific_suffix: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            image: Ok(Default::default()),
            name: Ok(Default::default()),
            official: Ok(Default::default()),
            other_identifier: Ok(Default::default()),
            parent_org: Ok(Default::default()),
            patronymic_name: Ok(Default::default()),
            phone: Ok(Default::default()),
            type_: Err("no value supplied for type_".to_string()),
            url: Ok(Default::default()),
        }
    }
}
impl ProfileBuilder {
    pub fn additional_name<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.additional_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for additional_name: {}", e));
        self
    }
    pub fn address<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<Address>>,
        T::Error: std::fmt::Display,
    {
        self.address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address: {}", e));
        self
    }
    pub fn date_of_birth<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<chrono::NaiveDate>>,
        T::Error: std::fmt::Display,
    {
        self.date_of_birth = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for date_of_birth: {}", e));
        self
    }
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {}", e));
        self
    }
    pub fn email<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.email = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for email: {}", e));
        self
    }
    pub fn endorsement<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Vec<endorsement::EndorsementCredential>>,
        T::Error: std::fmt::Display,
    {
        self.endorsement = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for endorsement: {}", e));
        self
    }
    pub fn endorsement_jwt<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Vec<ProfileEndorsementJwtItem>>,
        T::Error: std::fmt::Display,
    {
        self.endorsement_jwt = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for endorsement_jwt: {}", e));
        self
    }
    pub fn family_name<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.family_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for family_name: {}", e));
        self
    }
    pub fn family_name_prefix<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.family_name_prefix = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for family_name_prefix: {}", e));
        self
    }
    pub fn given_name<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.given_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for given_name: {}", e));
        self
    }
    pub fn honorific_prefix<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.honorific_prefix = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for honorific_prefix: {}", e));
        self
    }
    pub fn honorific_suffix<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.honorific_suffix = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for honorific_suffix: {}", e));
        self
    }
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {}", e));
        self
    }
    pub fn image<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<general::Image>>,
        T::Error: std::fmt::Display,
    {
        self.image = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for image: {}", e));
        self
    }
    pub fn name<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {}", e));
        self
    }
    pub fn official<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.official = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for official: {}", e));
        self
    }
    pub fn other_identifier<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Vec<identity::IdentifierEntry>>,
        T::Error: std::fmt::Display,
    {
        self.other_identifier = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for other_identifier: {}", e));
        self
    }
    pub fn parent_org<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Box<Option<Profile>>>,
        T::Error: std::fmt::Display,
    {
        self.parent_org = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for parent_org: {}", e));
        self
    }
    pub fn patronymic_name<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.patronymic_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for patronymic_name: {}", e));
        self
    }
    pub fn phone<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.phone = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for phone: {}", e));
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<ProfileType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
    pub fn url<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.url = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for url: {}", e));
        self
    }
}
impl std::convert::TryFrom<ProfileBuilder> for Profile {
    type Error = String;
    fn try_from(value: ProfileBuilder) -> Result<Self, String> {
        Ok(Self {
            additional_name: value.additional_name?,
            address: value.address?,
            date_of_birth: value.date_of_birth?,
            description: value.description?,
            email: value.email?,
            endorsement: value.endorsement?,
            endorsement_jwt: value.endorsement_jwt?,
            family_name: value.family_name?,
            family_name_prefix: value.family_name_prefix?,
            given_name: value.given_name?,
            honorific_prefix: value.honorific_prefix?,
            honorific_suffix: value.honorific_suffix?,
            id: value.id?,
            image: value.image?,
            name: value.name?,
            official: value.official?,
            other_identifier: value.other_identifier?,
            parent_org: value.parent_org?,
            patronymic_name: value.patronymic_name?,
            phone: value.phone?,
            type_: value.type_?,
            url: value.url?,
        })
    }
}
impl From<Profile> for ProfileBuilder {
    fn from(value: Profile) -> Self {
        Self {
            additional_name: Ok(value.additional_name),
            address: Ok(value.address),
            date_of_birth: Ok(value.date_of_birth),
            description: Ok(value.description),
            email: Ok(value.email),
            endorsement: Ok(value.endorsement),
            endorsement_jwt: Ok(value.endorsement_jwt),
            family_name: Ok(value.family_name),
            family_name_prefix: Ok(value.family_name_prefix),
            given_name: Ok(value.given_name),
            honorific_prefix: Ok(value.honorific_prefix),
            honorific_suffix: Ok(value.honorific_suffix),
            id: Ok(value.id),
            image: Ok(value.image),
            name: Ok(value.name),
            official: Ok(value.official),
            other_identifier: Ok(value.other_identifier),
            parent_org: Ok(value.parent_org),
            patronymic_name: Ok(value.patronymic_name),
            phone: Ok(value.phone),
            type_: Ok(value.type_),
            url: Ok(value.url),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddressBuilder {
    address_country: Result<Option<String>, String>,
    address_country_code: Result<Option<String>, String>,
    address_locality: Result<Option<String>, String>,
    address_region: Result<Option<String>, String>,
    geo: Result<Option<GeoCoordinates>, String>,
    post_office_box_number: Result<Option<String>, String>,
    postal_code: Result<Option<String>, String>,
    street_address: Result<Option<String>, String>,
    type_: Result<AddressType, String>,
}
impl Default for AddressBuilder {
    fn default() -> Self {
        Self {
            address_country: Ok(Default::default()),
            address_country_code: Ok(Default::default()),
            address_locality: Ok(Default::default()),
            address_region: Ok(Default::default()),
            geo: Ok(Default::default()),
            post_office_box_number: Ok(Default::default()),
            postal_code: Ok(Default::default()),
            street_address: Ok(Default::default()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl AddressBuilder {
    pub fn address_country<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.address_country = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_country: {}", e));
        self
    }
    pub fn address_country_code<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.address_country_code = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_country_code: {}", e));
        self
    }
    pub fn address_locality<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.address_locality = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_locality: {}", e));
        self
    }
    pub fn address_region<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.address_region = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_region: {}", e));
        self
    }
    pub fn geo<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<GeoCoordinates>>,
        T::Error: std::fmt::Display,
    {
        self.geo = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for geo: {}", e));
        self
    }
    pub fn post_office_box_number<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.post_office_box_number = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for post_office_box_number: {}", e));
        self
    }
    pub fn postal_code<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.postal_code = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for postal_code: {}", e));
        self
    }
    pub fn street_address<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<Option<String>>,
        T::Error: std::fmt::Display,
    {
        self.street_address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for street_address: {}", e));
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<AddressType>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
}
impl std::convert::TryFrom<AddressBuilder> for Address {
    type Error = String;
    fn try_from(value: AddressBuilder) -> Result<Self, String> {
        Ok(Self {
            address_country: value.address_country?,
            address_country_code: value.address_country_code?,
            address_locality: value.address_locality?,
            address_region: value.address_region?,
            geo: value.geo?,
            post_office_box_number: value.post_office_box_number?,
            postal_code: value.postal_code?,
            street_address: value.street_address?,
            type_: value.type_?,
        })
    }
}
impl From<Address> for AddressBuilder {
    fn from(value: Address) -> Self {
        Self {
            address_country: Ok(value.address_country),
            address_country_code: Ok(value.address_country_code),
            address_locality: Ok(value.address_locality),
            address_region: Ok(value.address_region),
            geo: Ok(value.geo),
            post_office_box_number: Ok(value.post_office_box_number),
            postal_code: Ok(value.postal_code),
            street_address: Ok(value.street_address),
            type_: Ok(value.type_),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeoCoordinatesBuilder {
    latitude: Result<f64, String>,
    longitude: Result<f64, String>,
    type_: Result<String, String>,
}
impl Default for GeoCoordinatesBuilder {
    fn default() -> Self {
        Self {
            latitude: Err("no value supplied for latitude".to_string()),
            longitude: Err("no value supplied for longitude".to_string()),
            type_: Err("no value supplied for type_".to_string()),
        }
    }
}
impl GeoCoordinatesBuilder {
    pub fn latitude<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<f64>,
        T::Error: std::fmt::Display,
    {
        self.latitude = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for latitude: {}", e));
        self
    }
    pub fn longitude<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<f64>,
        T::Error: std::fmt::Display,
    {
        self.longitude = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for longitude: {}", e));
        self
    }
    pub fn type_<T>(mut self, value: T) -> Self
    where
        T: std::convert::TryInto<String>,
        T::Error: std::fmt::Display,
    {
        self.type_ = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for type_: {}", e));
        self
    }
}
impl std::convert::TryFrom<GeoCoordinatesBuilder> for GeoCoordinates {
    type Error = String;
    fn try_from(value: GeoCoordinatesBuilder) -> Result<Self, String> {
        Ok(Self {
            latitude: value.latitude?,
            longitude: value.longitude?,
            type_: value.type_?,
        })
    }
}
impl From<GeoCoordinates> for GeoCoordinatesBuilder {
    fn from(value: GeoCoordinates) -> Self {
        Self {
            latitude: Ok(value.latitude),
            longitude: Ok(value.longitude),
            type_: Ok(value.type_),
        }
    }
}
