use std::borrow::Cow;

use time::Tm;

use chrono::{DateTime, TimeZone};
use {Cookie, SameSite};

/// Structure that follows the builder pattern for building `Cookie` structs.
///
/// To construct a cookie:
///
///   1. Call [`Cookie::build`](struct.Cookie.html#method.build) to start building.
///   2. Use any of the builder methods to set fields in the cookie.
///   3. Call [finish](#method.finish) to retrieve the built cookie.
///
/// # Example
///
/// ```rust
/// # extern crate cookie;
/// extern crate time;
///
/// use cookie::Cookie;
/// use time::Duration;
///
/// # fn main() {
/// let cookie: Cookie = Cookie::build("name", "value")
///     .domain("www.rust-lang.org")
///     .path("/")
///     .secure(true)
///     .http_only(true)
///     .max_age(60 * 60 * 24)
///     .finish();
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CookieBuilder<'c> {
    /// The cookie being built.
    cookie: Cookie<'c>,
}

impl<'c> CookieBuilder<'c> {
    /// Creates a new `CookieBuilder` instance from the given name and value.
    ///
    /// This method is typically called indirectly via
    /// [Cookie::build](struct.Cookie.html#method.build).
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Cookie;
    ///
    /// let c = Cookie::build("foo", "bar").finish();
    /// assert_eq!(c.name_value(), ("foo", "bar"));
    /// ```
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<Cow<'c, str>>,
        V: Into<Cow<'c, str>>,
    {
        CookieBuilder {
            cookie: Cookie::new(name, value),
        }
    }

    /// This function is deprecated, please use the expire_ts() or expire_datetime() functions instead.
    #[deprecated(
        since = "0.13.1",
        note = "Use the expire_ts() or expire_datetime() functions instead. It makes the interface independent from time crate. "
    )]
    pub fn expires(mut self, when: Tm) -> Self {
        self.cookie.set_expires(when);
        self
    }

    /// Sets the `expires` field in the cookie being built with a timestamp.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate cookie;
    /// extern crate chrono;
    ///
    /// use cookie::Cookie;
    ///
    /// # fn main() {
    /// let c = Cookie::build("foo", "bar")
    ///     .expires_ts(chrono::Local::now().timestamp())
    ///     .finish();
    ///
    /// assert!(c.expires().is_some());
    /// # }
    /// ```
    #[inline]
    pub fn expires_ts(mut self, when_in_sec: i64) -> Self {
        let tm: time::Tm = time::at_utc(time::Timespec::new(when_in_sec, 0));
        self.cookie.set_expires(tm);
        self
    }

    /// Sets the `expires` field in the cookie being built with a Utc DateTime.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate cookie;
    ///
    /// extern crate chrono;
    /// use cookie::Cookie;
    ///
    /// # fn main() {
    /// let c = Cookie::build("foo", "bar")
    ///     .expires_datetime(chrono::Utc::now())
    ///     .finish();
    ///
    /// assert!(c.expires().is_some());
    /// # }
    /// ```
    #[inline]
    pub fn expires_datetime<T>(self, when: DateTime<T>) -> Self
    where
        T: TimeZone,
    {
        self.expires_ts(when.timestamp())
    }

    /// Sets the `max_age` field in the cookie being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate cookie;
    /// extern crate time;
    /// use time::Duration;
    ///
    /// use cookie::Cookie;
    ///
    /// # fn main() {
    /// let c = Cookie::build("foo", "bar")
    ///     .max_age(60 * 30) // 30 minutes
    ///     .finish();
    ///
    /// assert_eq!(c.max_age(), Some(60 * 30));
    /// # }
    /// ```
    #[inline]
    pub fn max_age(mut self, value: u64) -> Self {
        self.cookie.set_max_age(value);
        self
    }

    /// Sets the `domain` field in the cookie being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Cookie;
    ///
    /// let c = Cookie::build("foo", "bar")
    ///     .domain("www.rust-lang.org")
    ///     .finish();
    ///
    /// assert_eq!(c.domain(), Some("www.rust-lang.org"));
    /// ```
    pub fn domain<D: Into<Cow<'c, str>>>(mut self, value: D) -> Self {
        self.cookie.set_domain(value);
        self
    }

    /// Sets the `path` field in the cookie being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Cookie;
    ///
    /// let c = Cookie::build("foo", "bar")
    ///     .path("/")
    ///     .finish();
    ///
    /// assert_eq!(c.path(), Some("/"));
    /// ```
    pub fn path<P: Into<Cow<'c, str>>>(mut self, path: P) -> Self {
        self.cookie.set_path(path);
        self
    }

    /// Sets the `secure` field in the cookie being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Cookie;
    ///
    /// let c = Cookie::build("foo", "bar")
    ///     .secure(true)
    ///     .finish();
    ///
    /// assert_eq!(c.secure(), Some(true));
    /// ```
    #[inline]
    pub fn secure(mut self, value: bool) -> Self {
        self.cookie.set_secure(value);
        self
    }

    /// Sets the `http_only` field in the cookie being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Cookie;
    ///
    /// let c = Cookie::build("foo", "bar")
    ///     .http_only(true)
    ///     .finish();
    ///
    /// assert_eq!(c.http_only(), Some(true));
    /// ```
    #[inline]
    pub fn http_only(mut self, value: bool) -> Self {
        self.cookie.set_http_only(value);
        self
    }

    /// Sets the `same_site` field in the cookie being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::{Cookie, SameSite};
    ///
    /// let c = Cookie::build("foo", "bar")
    ///     .same_site(SameSite::Strict)
    ///     .finish();
    ///
    /// assert_eq!(c.same_site(), Some(SameSite::Strict));
    /// ```
    #[inline]
    pub fn same_site(mut self, value: SameSite) -> Self {
        self.cookie.set_same_site(value);
        self
    }

    /// Makes the cookie being built 'permanent' by extending its expiration and
    /// max age 20 years into the future.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate cookie;
    /// extern crate time;
    ///
    /// use cookie::Cookie;
    /// use time::Duration;
    ///
    /// # fn main() {
    /// let c = Cookie::build("foo", "bar")
    ///     .permanent()
    ///     .finish();
    ///
    /// let twenty_years = 60 * 60 * 24 * 365 * 20;
    /// assert_eq!(c.max_age(), Some(twenty_years));
    /// # assert!(c.expires().is_some());
    /// # }
    /// ```
    #[inline]
    pub fn permanent(mut self) -> Self {
        self.cookie.make_permanent();
        self
    }

    /// Finishes building and returns the built `Cookie`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Cookie;
    ///
    /// let c = Cookie::build("foo", "bar")
    ///     .domain("crates.io")
    ///     .path("/")
    ///     .finish();
    ///
    /// assert_eq!(c.name_value(), ("foo", "bar"));
    /// assert_eq!(c.domain(), Some("crates.io"));
    /// assert_eq!(c.path(), Some("/"));
    /// ```
    #[inline]
    pub fn finish(self) -> Cookie<'c> {
        self.cookie
    }
}
