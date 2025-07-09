use gh_gist::{
    ENV_API_HOST, ENV_TOKEN, GistClient,
    models::{CreateComment, CreateGist, GistFile, UpdateComment},
};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() {
    // 初始化客户端
    // let client = GistClient::new(
    //     Some("".to_string()),
    //     None, // 使用默认 API 主机
    // );
    unsafe {
        env::set_var(ENV_API_HOST, "https://upxgo.deno.dev/gh");
        env::set_var(ENV_TOKEN, "");
    }

    let client = GistClient::from_env();

    println!(
        "{}:{}",
        client.api_host,
        client.token.clone().unwrap_or_default()
    );

    // 创建新 Gist
    let mut files = HashMap::new();
    files.insert(
        "hello.rs".to_string(),
        GistFile {
            content: Some("fn main() { println!(\"Hello World!\"); }".to_string()),
            filename: Some("hello.rs".to_string()),
            ..Default::default()
        },
    );

    let new_gist = CreateGist {
        description: Some("Rust Hello World".to_string()),
        public: Some(true),
        files,
    };

    match client.create_gist(&new_gist).await {
        Ok(gist) => {
            println!("Created Gist: {}", gist.html_url);
            let gist_id = gist.id;

            // 添加评论
            let comment = CreateComment {
                body: "This is a great code example!".to_string(),
            };
            let created_comment = client
                .create_gist_comment(&gist_id, &comment)
                .await
                .unwrap();
            println!("Added comment: {}", created_comment.id);

            // 更新评论
            let update = UpdateComment {
                body: "Updated comment content".to_string(),
            };
            client
                .update_gist_comment(&gist_id, created_comment.id, &update)
                .await
                .unwrap();

            // 获取提交历史
            let commits = client.list_gist_commits(&gist_id).await.unwrap();
            println!("Gist has {} commits", commits.len());

            // 获取特定版本
            if let Some(commit) = commits.first() {
                let revision = client
                    .get_gist_revision(&gist_id, &commit.version)
                    .await
                    .unwrap();
                println!("Revision created at: {}", revision.created_at);
            }

            // 分叉 Gist
            // let fork = client.fork_gist(&gist_id).await.unwrap();
            // println!("Forked Gist: {}", fork.id);

            // 星标 Gist
            client.star_gist(&gist_id).await.unwrap();
            let is_starred = client.check_gist_star(&gist_id).await.unwrap();
            println!("Gist is starred: {}", is_starred);

            // 清理
            // client.delete_gist(&gist_id).await.unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
