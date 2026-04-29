use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub owner: User,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub forks: i32,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse {
    pub total_count: i32,
    pub incomplete_results: bool,
    pub items: Vec<Repo>,
}

pub async fn fetch_repositories() -> Result<Vec<Repo>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("u2020dx-app")
        .build()?;

    let url = "https://api.github.com/search/repositories?q=created&sort=stars&order=desc";
    let response = client
        .get(url)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;

    Ok(response.items)
}

/*
class User {
  final int id;
  final String login;
  final String avatarUrl;
  final String url;
  final String htmlUrl;

  const User({this.id, this.login, this.avatarUrl, this.url, this.htmlUrl});

  User.fromMap(Map<String, dynamic> map) :
        id = map['id'],
        login = map['login'],
        avatarUrl = map['avatar_url'],
        url = map['url'],
        htmlUrl = map['html_url'];


}

        class Repo {

  final int id;
  final String name;
  final String fullName;
  final String htmlUrl;

  //add this
  final User owner;
  final int stargazersCount;
  final int watchersCount;
  final int forks;
  final String language;

  const Repo(
      {this.id, this.name, this.fullName, this.htmlUrl, this.stargazersCount,
      this.watchersCount, this.forks, this.language, this.owner});

  Repo.fromMap(Map<String, dynamic> map) :
      id = map['id'],
      name = map['name'],
      fullName = map['full_name'],
      htmlUrl = map['html_url'],
      stargazersCount = map['stargazers_count'],
      watchersCount = map['watchers_count'],
      forks = map['forks'],
      language = map['language'],
      owner = new User.fromMap(map['owner']);

}

abstract class RepoRepository {
  Future<List<Repo>> fetch();
}

class FetchDataException implements Exception {
  String _message;

  FetchDataException(this._message);

  @override
  String toString() {
    return 'Exception: $_message';
  }

}
*/
