use sea_orm::Statement;
use sea_orm_migration::{prelude::*, sea_orm::ConnectionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // CREATE TABLES //
        // AcceptedTerms.
        manager
            .create_table(
                Table::create()
                    .table(AcceptedTerms::Table)
                    .col(
                        ColumnDef::new(AcceptedTerms::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AcceptedTerms::UsersId).unsigned().not_null())
                    .col(ColumnDef::new(AcceptedTerms::TermsId).unsigned().not_null())
                    .col(
                        ColumnDef::new(AcceptedTerms::Revision)
                            .unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // AccountTypes.
        manager
            .create_table(
                Table::create()
                    .table(AccountTypes::Table)
                    .col(
                        ColumnDef::new(AccountTypes::Id)
                            .tiny_unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AccountTypes::AccountType)
                            .string_len(32)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // ApiKeys.
        manager
            .create_table(
                Table::create()
                    .table(ApiKeys::Table)
                    .col(
                        ColumnDef::new(ApiKeys::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ApiKeys::UserId).unsigned().not_null())
                    .col(
                        ColumnDef::new(ApiKeys::KeyHash)
                            .text()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ApiKeys::Note).text().null())
                    .col(ColumnDef::new(ApiKeys::ExpireTs).big_unsigned().null())
                    .to_owned(),
            )
            .await?;

        // ApiRateLimit.
        manager
            .create_table(
                Table::create()
                    .table(ApiRateLimit::Table)
                    .col(
                        ColumnDef::new(ApiRateLimit::Ip)
                            .string_len(45)
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ApiRateLimit::Requests).integer().not_null())
                    .col(
                        ColumnDef::new(ApiRateLimit::WindowStart)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Bans.
        manager
            .create_table(
                Table::create()
                    .table(Bans::Table)
                    .col(
                        ColumnDef::new(Bans::IpAddress)
                            .string_len(45)
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bans::BanTs).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // DependencyTypes.
        manager
            .create_table(
                Table::create()
                    .table(DependencyTypes::Table)
                    .col(
                        ColumnDef::new(DependencyTypes::Id)
                            .tiny_unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DependencyTypes::Name)
                            .string_len(32)
                            .string_len(32)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Licenses.
        manager
            .create_table(
                Table::create()
                    .table(Licenses::Table)
                    .col(
                        ColumnDef::new(Licenses::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Licenses::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // OfficialProviders.
        manager
            .create_table(
                Table::create()
                    .table(OfficialProviders::Table)
                    .col(
                        ColumnDef::new(OfficialProviders::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OfficialProviders::Name)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OfficialProviders::Repo)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OfficialProviders::Provides)
                            .string_len(64)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // PackageBases.
        manager
            .create_table(
                Table::create()
                    .table(PackageBases::Table)
                    .col(
                        ColumnDef::new(PackageBases::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PackageBases::NumVotes).unsigned().not_null())
                    .col(
                        ColumnDef::new(PackageBases::Popularity)
                            .decimal_len(10, 6)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::OutOfDateTs)
                            .big_unsigned_len(20)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::FlaggerComment)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::SubmittedTs)
                            .big_unsigned_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::ModifiedTs)
                            .big_unsigned_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::RepologyCheck)
                            .tiny_unsigned_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::NumGitPulls)
                            .big_unsigned_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::FlaggerUid)
                            .unsigned_len(10)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::SubmitterUid)
                            .unsigned_len(10)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::MaintainerUid)
                            .unsigned_len(10)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PackageBases::PackagerUid)
                            .unsigned_len(10)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // PackageBlacklist.
        manager
            .create_table(
                Table::create()
                    .table(PackageBlacklist::Table)
                    .col(
                        ColumnDef::new(PackageBlacklist::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageBlacklist::Name)
                            .string_len(64)
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // PackageComaintainers.
        manager
            .create_table(
                Table::create()
                    .table(PackageComaintainers::Table)
                    .col(
                        ColumnDef::new(PackageComaintainers::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageComaintainers::UsersId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageComaintainers::PackageBaseId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageComaintainers::Priority)
                            .unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // PackageComments.
        manager
            .create_table(
                Table::create()
                    .table(PackageComments::Table)
                    .col(
                        ColumnDef::new(PackageComments::Id)
                            .big_unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageComments::PackageBaseId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PackageComments::UsersId).unsigned().null())
                    .col(ColumnDef::new(PackageComments::Comments).text().not_null())
                    .col(
                        ColumnDef::new(PackageComments::RenderedComment)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageComments::CommentTs)
                            .unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PackageComments::EditedTs).unsigned().null())
                    .col(
                        ColumnDef::new(PackageComments::EditedUsersId)
                            .unsigned()
                            .null(),
                    )
                    .col(ColumnDef::new(PackageComments::DelTs).big_unsigned().null())
                    .col(
                        ColumnDef::new(PackageComments::DelUsersId)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PackageComments::PinnedTs)
                            .big_unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // PackageDepends.
        manager
            .create_table(
                Table::create()
                    .table(PackageDepends::Table)
                    .col(
                        ColumnDef::new(PackageDepends::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageDepends::PackageId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageDepends::DepTypeId)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PackageDepends::DepName).string().not_null())
                    .col(ColumnDef::new(PackageDepends::DepDesc).string().null())
                    .col(ColumnDef::new(PackageDepends::DepCondition).string().null())
                    .col(ColumnDef::new(PackageDepends::DepArch).string().null())
                    .col(ColumnDef::new(PackageDepends::DepDist).string().null())
                    .to_owned(),
            )
            .await?;

        // PackageKeywords.
        // I can't figure out how to do composite keys with SeaORM directly yet, so use raw SQL for the time being.
        let statement = Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE PackageKeywords (
                PackageBaseID INT UNSIGNED,
                Keyword VARCHAR(255),
                PRIMARY KEY (PackageBaseID, Keyword)
            )"#
            .to_string(),
        );
        manager
            .get_connection()
            .execute(statement)
            .await
            .map(|_| ())?;

        // PackageLicenses.
        // I can't figure out how to do composite keys with SeaORM directly yet, so use raw SQL for the time being.
        let statement = Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE PackageLicenses (
                PackageID INT UNSIGNED,
                LicenseID INT UNSIGNED,
                PRIMARY KEY (PackageID, LicenseID)
            )"#
            .to_string(),
        );
        manager
            .get_connection()
            .execute(statement)
            .await
            .map(|_| ())?;

        // PackageNotifications.
        manager
            .create_table(
                Table::create()
                    .table(PackageNotifications::Table)
                    .col(
                        ColumnDef::new(PackageNotifications::Id)
                            .unsigned()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageNotifications::PackageBaseId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageNotifications::UserId)
                            .unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // PackageRelations.
        manager
            .create_table(
                Table::create()
                    .table(PackageRelations::Table)
                    .col(
                        ColumnDef::new(PackageRelations::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRelations::PackageId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRelations::RelTypeId)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRelations::RelName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRelations::RelCondition)
                            .string()
                            .null(),
                    )
                    .col(ColumnDef::new(PackageRelations::RelArch).string().null())
                    .col(ColumnDef::new(PackageRelations::RelDist).string().null())
                    .to_owned(),
            )
            .await?;

        // PackageRequests.
        manager
            .create_table(
                Table::create()
                    .table(PackageRequests::Table)
                    .col(
                        ColumnDef::new(PackageRequests::Id)
                            .big_unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRequests::ReqTypeId)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRequests::PackageBaseId)
                            .unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PackageRequests::PackageBaseName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRequests::MergeBaseName)
                            .string()
                            .null(),
                    )
                    .col(ColumnDef::new(PackageRequests::UsersId).unsigned().null())
                    .col(ColumnDef::new(PackageRequests::Comments).text().not_null())
                    .col(
                        ColumnDef::new(PackageRequests::ClosureComment)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRequests::RequestTs)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageRequests::ClosedTs)
                            .big_unsigned()
                            .null(),
                    )
                    .col(ColumnDef::new(PackageRequests::ClosedUid).unsigned().null())
                    .col(
                        ColumnDef::new(PackageRequests::Status)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // PackageSources.
        manager
            .create_table(
                Table::create()
                    .table(PackageSources::Table)
                    .col(
                        ColumnDef::new(PackageSources::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PackageSources::PackageId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PackageSources::Source).string().not_null())
                    .col(ColumnDef::new(PackageSources::SourceArch).string().null())
                    .col(ColumnDef::new(PackageSources::SourceDist).string().null())
                    .to_owned(),
            )
            .await?;

        // PackageVotes.
        manager
            .create_table(
                Table::create()
                    .table(PackageVotes::Table)
                    .col(
                        ColumnDef::new(PackageVotes::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PackageVotes::UsersId).unsigned().not_null())
                    .col(
                        ColumnDef::new(PackageVotes::PackageBaseId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PackageVotes::VoteTs).big_unsigned().null())
                    .to_owned(),
            )
            .await?;

        // Packages.
        manager
            .create_table(
                Table::create()
                    .table(Packages::Table)
                    .col(
                        ColumnDef::new(Packages::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Packages::PackageBaseId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Packages::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Packages::Version).string().not_null())
                    .col(ColumnDef::new(Packages::Description).string().null())
                    .col(ColumnDef::new(Packages::Url).string().null())
                    .to_owned(),
            )
            .await?;

        // RelationTypes.
        manager
            .create_table(
                Table::create()
                    .table(RelationTypes::Table)
                    .col(
                        ColumnDef::new(RelationTypes::Id)
                            .tiny_unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RelationTypes::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // RequestTypes.
        manager
            .create_table(
                Table::create()
                    .table(RequestTypes::Table)
                    .col(
                        ColumnDef::new(RequestTypes::Id)
                            .tiny_unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RequestTypes::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Sessions.
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .col(
                        ColumnDef::new(Sessions::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Sessions::UsersId).unsigned().not_null())
                    .col(
                        ColumnDef::new(Sessions::SessionId)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Sessions::LastUpdateTs)
                            .big_unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // SSHPubKeys.
        manager
            .create_table(
                Table::create()
                    .table(SshPubKeys::Table)
                    .col(ColumnDef::new(SshPubKeys::UserId).unsigned().not_null())
                    .col(
                        ColumnDef::new(SshPubKeys::Fingerprint)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SshPubKeys::PubKey).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Terms.
        manager
            .create_table(
                Table::create()
                    .table(Terms::Table)
                    .col(
                        ColumnDef::new(Terms::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Terms::Description).string().not_null())
                    .col(ColumnDef::new(Terms::Url).string().not_null())
                    .col(ColumnDef::new(Terms::Revision).unsigned().not_null())
                    .to_owned(),
            )
            .await?;

        // TU_VoteInfo.
        manager
            .create_table(
                Table::create()
                    .table(TuVoteInfo::Table)
                    .col(
                        ColumnDef::new(TuVoteInfo::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TuVoteInfo::Agenda).text().not_null())
                    .col(ColumnDef::new(TuVoteInfo::User).string().not_null())
                    .col(
                        ColumnDef::new(TuVoteInfo::Submitted)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TuVoteInfo::End).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(TuVoteInfo::Quorum)
                            .decimal_len(2, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TuVoteInfo::SubmitterId)
                            .unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TuVoteInfo::Yes).unsigned().not_null())
                    .col(ColumnDef::new(TuVoteInfo::No).unsigned().not_null())
                    .col(ColumnDef::new(TuVoteInfo::Abstain).unsigned().not_null())
                    .col(ColumnDef::new(TuVoteInfo::ActiveTus).unsigned().not_null())
                    .to_owned(),
            )
            .await?;

        // TU_Votes.
        manager
            .create_table(
                Table::create()
                    .table(TuVotes::Table)
                    .col(
                        ColumnDef::new(TuVotes::Id)
                            .unsigned()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TuVotes::VoteId).unsigned().not_null())
                    .col(ColumnDef::new(TuVotes::UserId).unsigned().not_null())
                    .to_owned(),
            )
            .await?;

        // Users.
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .unsigned()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::AccountTypeId)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Suspended).tiny_unsigned().not_null())
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::BackupEmail).string().null())
                    .col(ColumnDef::new(Users::HideEmail).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Users::Passwd).string().not_null())
                    .col(ColumnDef::new(Users::Salt).string().not_null())
                    .col(ColumnDef::new(Users::ResetKey).string().not_null())
                    .col(ColumnDef::new(Users::RealName).string().not_null())
                    .col(ColumnDef::new(Users::LangPreference).string().not_null())
                    .col(ColumnDef::new(Users::Timezone).string().not_null())
                    .col(ColumnDef::new(Users::Homepage).text().null())
                    .col(ColumnDef::new(Users::IrcNick).string().not_null())
                    .col(ColumnDef::new(Users::PgpKey).string().null())
                    .col(ColumnDef::new(Users::LastLogin).big_unsigned().not_null())
                    .col(ColumnDef::new(Users::LastLoginIpAddress).string().null())
                    .col(
                        ColumnDef::new(Users::LastSshLogin)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::LastSshLoginIpAddress).string().null())
                    .col(
                        ColumnDef::new(Users::InactivityTs)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::RegistrationTs).timestamp().not_null())
                    .col(
                        ColumnDef::new(Users::CommentNotify)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdateNotify)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::OwnershipNotify)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::SsoAccountId)
                            .string()
                            .unique_key()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // SET UP RELATIONSHIPS. //
        // AcceptedTerms.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("AcceptedTermsUsersId")
                    .from(AcceptedTerms::Table, AcceptedTerms::UsersId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("AcceptedTermsTermsId")
                    .from(AcceptedTerms::Table, AcceptedTerms::TermsId)
                    .to(Terms::Table, Terms::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // ApiKeys.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("ApiKeysUserId")
                    .from(ApiKeys::Table, ApiKeys::UserId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // PackageBases.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageBasesFlaggerUid")
                    .from(PackageBases::Table, PackageBases::FlaggerUid)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageBasesSubmitterUid")
                    .from(PackageBases::Table, PackageBases::SubmitterUid)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageBasesMaintainerUid")
                    .from(PackageBases::Table, PackageBases::MaintainerUid)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageBasesPackagerUid")
                    .from(PackageBases::Table, PackageBases::PackagerUid)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // PackageComaintainers.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageComaintainersUsersId")
                    .from(PackageComaintainers::Table, PackageComaintainers::UsersId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageComaintainerPackageBaseId")
                    .from(
                        PackageComaintainers::Table,
                        PackageComaintainers::PackageBaseId,
                    )
                    .to(PackageBases::Table, PackageBases::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // PackageComments.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageCommentsPackageBaseId")
                    .from(PackageComments::Table, PackageComments::PackageBaseId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageCommentsUsersId")
                    .from(PackageComments::Table, PackageComments::UsersId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageCommentsEditedUsersId")
                    .from(PackageComments::Table, PackageComments::EditedUsersId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageCommentsDelUsersId")
                    .from(PackageComments::Table, PackageComments::DelUsersId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // PackageDepends.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageDependsPackageId")
                    .from(PackageDepends::Table, PackageDepends::PackageId)
                    .to(Packages::Table, Packages::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageDependsDepTypeId")
                    .from(PackageDepends::Table, PackageDepends::DepTypeId)
                    .to(DependencyTypes::Table, DependencyTypes::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        // PackageKeywords.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageKeywordsPackageBaseId")
                    .from(PackageKeywords::Table, PackageKeywords::PackageBaseId)
                    .to(PackageBases::Table, PackageBases::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // PackageLicenses.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageLicencesPackageId")
                    .from(PackageLicenses::Table, PackageLicenses::PackageId)
                    .to(Packages::Table, Packages::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageLicensesLicenseId")
                    .from(PackageLicenses::Table, PackageLicenses::LicenseId)
                    .to(Licenses::Table, Licenses::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // PacakgeNotifications.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageNotificationsPackageBaseId")
                    .from(
                        PackageNotifications::Table,
                        PackageNotifications::PackageBaseId,
                    )
                    .to(PackageBases::Table, PackageBases::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageNotificationsUserId")
                    .from(PackageNotifications::Table, PackageNotifications::UserId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // PackageRelations.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageRelationsPackageId")
                    .from(PackageRelations::Table, PackageRelations::PackageId)
                    .to(Packages::Table, Packages::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageRelationsRelTypeId")
                    .from(PackageRelations::Table, PackageRelations::RelTypeId)
                    .to(RelationTypes::Table, RelationTypes::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        // PackageRequests.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageRequestsReqTypeId")
                    .from(PackageRequests::Table, PackageRequests::ReqTypeId)
                    .to(RequestTypes::Table, RequestTypes::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageRequestsPackageBaseId")
                    .from(PackageRequests::Table, PackageRequests::PackageBaseId)
                    .to(PackageBases::Table, PackageBases::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageRequestsUsersId")
                    .from(PackageRequests::Table, PackageRequests::UsersId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageRequestsClosedUid")
                    .from(PackageRequests::Table, PackageRequests::ClosedUid)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // PackageSources.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageSourcesPackageId")
                    .from(PackageSources::Table, PackageSources::PackageId)
                    .to(Packages::Table, Packages::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // PackageVotes.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageVotesUsersId")
                    .from(PackageVotes::Table, PackageVotes::UsersId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackageVotesPackageBaseId")
                    .from(PackageVotes::Table, PackageVotes::PackageBaseId)
                    .to(PackageBases::Table, PackageBases::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Packages.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("PackagesPackageBaseId")
                    .from(Packages::Table, Packages::PackageBaseId)
                    .to(PackageBases::Table, PackageBases::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Sessions.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("SessionsUsersId")
                    .from(Sessions::Table, Sessions::UsersId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // SSHPubKeys.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("SshPubKeysUserId")
                    .from(SshPubKeys::Table, SshPubKeys::UserId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // TU_VoteInfo.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("TuVoteInfoSubmitterId")
                    .from(TuVoteInfo::Table, TuVoteInfo::SubmitterId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // TU_Votes.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("TuVotesVoteId")
                    .from(TuVotes::Table, TuVotes::VoteId)
                    .to(TuVoteInfo::Table, TuVoteInfo::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("TuVotesUserId")
                    .from(TuVotes::Table, TuVotes::UserId)
                    .to(Users::Table, Users::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Users.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("UsersAccountTypeId")
                    .from(Users::Table, Users::AccountTypeId)
                    .to(AccountTypes::Table, AccountTypes::Id)
                    .on_update(ForeignKeyAction::Restrict)
                    .on_delete(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Delete foreign key constraints.
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(AcceptedTerms::Table)
                    .name("AcceptedTermsUsersId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(AcceptedTerms::Table)
                    .name("AcceptedTermsTermsId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(ApiKeys::Table)
                    .name("ApiKeysUserId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageBases::Table)
                    .name("PackageBasesFlaggerUid")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageBases::Table)
                    .name("PackageBasesSubmitterUid")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageBases::Table)
                    .name("PackageBasesMaintainerUid")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageBases::Table)
                    .name("PackageBasesPackagerUid")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageComaintainers::Table)
                    .name("PackageComaintainersUsersId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageComaintainers::Table)
                    .name("PackageComaintainerPackageBaseId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageComments::Table)
                    .name("PackageCommentsPackageBaseId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageComments::Table)
                    .name("PackageCommentsUsersId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageComments::Table)
                    .name("PackageCommentsEditedUsersId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageComments::Table)
                    .name("PackageCommentsDelUsersId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageDepends::Table)
                    .name("PackageDependsPackageId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageDepends::Table)
                    .name("PackageDependsDepTypeId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageKeywords::Table)
                    .name("PackageKeywordsPackageBaseId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageLicenses::Table)
                    .name("PackageLicencesPackageId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageLicenses::Table)
                    .name("PackageLicensesLicenseId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageNotifications::Table)
                    .name("PackageNotificationsPackageBaseId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageNotifications::Table)
                    .name("PackageNotificationsUserId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageRelations::Table)
                    .name("PackageRelationsPackageId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageRelations::Table)
                    .name("PackageRelationsRelTypeId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageRequests::Table)
                    .name("PackageRequestsReqTypeId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageRequests::Table)
                    .name("PackageRequestsPackageBaseId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageRequests::Table)
                    .name("PackageRequestsUsersId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageRequests::Table)
                    .name("PackageRequestsClosedUid")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageSources::Table)
                    .name("PackageSourcesPackageId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageVotes::Table)
                    .name("PackageVotesUsersId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PackageVotes::Table)
                    .name("PackageVotesPackageBaseId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Packages::Table)
                    .name("PackagesPackageBaseId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Sessions::Table)
                    .name("SessionsUsersId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(SshPubKeys::Table)
                    .name("SshPubKeysUserId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(TuVoteInfo::Table)
                    .name("TuVoteInfoSubmitterId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(TuVotes::Table)
                    .name("TuVotesVoteId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(TuVotes::Table)
                    .name("TuVotesUserId")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Users::Table)
                    .name("UsersAccountTypeId")
                    .to_owned(),
            )
            .await?;

        // Delete tables.
        manager
            .drop_table(Table::drop().table(AcceptedTerms::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AccountTypes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ApiKeys::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ApiRateLimit::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Bans::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DependencyTypes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Licenses::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OfficialProviders::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageBases::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageBlacklist::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageComaintainers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageComments::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageDepends::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageKeywords::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageLicenses::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageNotifications::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageRelations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageRequests::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageSources::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PackageVotes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Packages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RelationTypes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RequestTypes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SshPubKeys::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Terms::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TuVoteInfo::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TuVotes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum AcceptedTerms {
    #[iden = "AcceptedTerms"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "UsersID"]
    UsersId,
    #[iden = "TermsID"]
    TermsId,
    #[iden = "Revision"]
    Revision,
}

#[derive(Iden)]
enum AccountTypes {
    #[iden = "AccountTypes"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "AccountType"]
    AccountType,
}
// #[derive(Iden)]
// enum AlembicVersion {
//     #[iden = "alembic_version"]
//     Table,
//     #[iden = "version_num"]
//     VersionNum,
// }

#[derive(Iden)]
enum ApiKeys {
    #[iden = "ApiKeys"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "UserID"]
    UserId,
    #[iden = "KeyHash"]
    KeyHash,
    #[iden = "Note"]
    Note,
    #[iden = "ExpireTS"]
    ExpireTs,
}

#[derive(Iden)]
enum ApiRateLimit {
    #[iden = "ApiRateLimit"]
    Table,
    #[iden = "IP"]
    Ip,
    #[iden = "Requests"]
    Requests,
    #[iden = "WindowStart"]
    WindowStart,
}

#[derive(Iden)]
enum Bans {
    #[iden = "Bans"]
    Table,
    #[iden = "IPAddress"]
    IpAddress,
    #[iden = "BanTS"]
    BanTs,
}

#[derive(Iden)]
enum DependencyTypes {
    #[iden = "DependencyTypes"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Name"]
    Name,
}

#[derive(Iden)]
enum Licenses {
    #[iden = "Licenses"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Name"]
    Name,
}

#[derive(Iden)]
enum OfficialProviders {
    #[iden = "OfficialProviders"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Name"]
    Name,
    #[iden = "Repo"]
    Repo,
    #[iden = "Provides"]
    Provides,
}

#[derive(Iden)]
enum PackageBases {
    #[iden = "PackageBases"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Name"]
    Name,
    #[iden = "NumVotes"]
    NumVotes,
    #[iden = "Popularity"]
    Popularity,
    #[iden = "OutOfDateTS"]
    OutOfDateTs,
    #[iden = "FlaggerComment"]
    FlaggerComment,
    #[iden = "SubmittedTS"]
    SubmittedTs,
    #[iden = "ModifiedTS"]
    ModifiedTs,
    #[iden = "RepologyCheck"]
    RepologyCheck,
    #[iden = "NumGitPulls"]
    NumGitPulls,
    #[iden = "FlaggerUID"]
    FlaggerUid,
    #[iden = "SubmitterUID"]
    SubmitterUid,
    #[iden = "MaintainerUID"]
    MaintainerUid,
    #[iden = "PackagerUID"]
    PackagerUid,
}

#[derive(Iden)]
enum PackageBlacklist {
    #[iden = "PackageBlacklist"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Name"]
    Name,
}

#[derive(Iden)]
enum PackageComaintainers {
    #[iden = "PackageComaintainers"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "UsersID"]
    UsersId,
    #[iden = "PackageBaseID"]
    PackageBaseId,
    #[iden = "Priority"]
    Priority,
}

#[derive(Iden)]
enum PackageComments {
    #[iden = "PackageComments"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "PackageBaseID"]
    PackageBaseId,
    #[iden = "UsersID"]
    UsersId,
    #[iden = "Comments"]
    Comments,
    #[iden = "RenderedComment"]
    RenderedComment,
    #[iden = "CommentTS"]
    CommentTs,
    #[iden = "EditedTS"]
    EditedTs,
    #[iden = "EditedUsersID"]
    EditedUsersId,
    #[iden = "DelTS"]
    DelTs,
    #[iden = "DelUsersID"]
    DelUsersId,
    #[iden = "PinnedTS"]
    PinnedTs,
}
#[derive(Iden)]
enum PackageDepends {
    #[iden = "PackageDepends"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "PackageID"]
    PackageId,
    #[iden = "DepTypeId"]
    DepTypeId,
    #[iden = "DepName"]
    DepName,
    #[iden = "DepDesc"]
    DepDesc,
    #[iden = "DepCondition"]
    DepCondition,
    #[iden = "DepArch"]
    DepArch,
    #[iden = "DepDist"]
    DepDist,
}

#[derive(Iden)]
enum PackageKeywords {
    #[iden = "PackageKeywords"]
    Table,
    #[iden = "PackageBaseID"]
    PackageBaseId,
}

#[derive(Iden)]
enum PackageLicenses {
    #[iden = "PackageLicenses"]
    Table,
    #[iden = "PackageID"]
    PackageId,
    #[iden = "LicenseID"]
    LicenseId,
}

#[derive(Iden)]
enum PackageNotifications {
    #[iden = "PackageNotifications"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "PackageBaseID"]
    PackageBaseId,
    #[iden = "UserID"]
    UserId,
}

#[derive(Iden)]
enum PackageRelations {
    #[iden = "PackageRelations"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "PackageID"]
    PackageId,
    #[iden = "RelTypeID"]
    RelTypeId,
    #[iden = "RelName"]
    RelName,
    #[iden = "RelCondition"]
    RelCondition,
    #[iden = "RelArch"]
    RelArch,
    #[iden = "RelDist"]
    RelDist,
}

#[derive(Iden)]
enum PackageRequests {
    #[iden = "PackageRequests"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "ReqTypeID"]
    ReqTypeId,
    #[iden = "PackageBaseID"]
    PackageBaseId,
    #[iden = "PackageBaseName"]
    PackageBaseName,
    #[iden = "MergeBaseName"]
    MergeBaseName,
    #[iden = "UsersID"]
    UsersId,
    #[iden = "Comments"]
    Comments,
    #[iden = "ClosureComment"]
    ClosureComment,
    #[iden = "RequestTS"]
    RequestTs,
    #[iden = "ClosedTS"]
    ClosedTs,
    #[iden = "ClosedUID"]
    ClosedUid,
    #[iden = "Status"]
    Status,
}

#[derive(Iden)]
enum PackageSources {
    #[iden = "PackageSources"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "PackageID"]
    PackageId,
    #[iden = "Source"]
    Source,
    #[iden = "SourceArch"]
    SourceArch,
    #[iden = "SourceDist"]
    SourceDist,
}

#[derive(Iden)]
enum PackageVotes {
    #[iden = "PackageVotes"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "UsersID"]
    UsersId,
    #[iden = "PackageBaseID"]
    PackageBaseId,
    #[iden = "VoteTS"]
    VoteTs,
}

#[derive(Iden)]
enum Packages {
    #[iden = "Packages"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "PackageBaseID"]
    PackageBaseId,
    #[iden = "Name"]
    Name,
    #[iden = "Version"]
    Version,
    #[iden = "Description"]
    Description,
    #[iden = "URL"]
    Url,
}

#[derive(Iden)]
enum RelationTypes {
    #[iden = "RelationTypes"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Name"]
    Name,
}

#[derive(Iden)]
enum RequestTypes {
    #[iden = "RequestTypes"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Name"]
    Name,
}

#[derive(Iden)]
enum Sessions {
    #[iden = "Sessions"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "UsersID"]
    UsersId,
    #[iden = "SessionID"]
    SessionId,
    #[iden = "LastUpdateTS"]
    LastUpdateTs,
}

#[derive(Iden)]
enum SshPubKeys {
    #[iden = "SSHPubKeys"]
    Table,
    #[iden = "UserID"]
    UserId,
    #[iden = "Fingerprint"]
    Fingerprint,
    #[iden = "PubKey"]
    PubKey,
}

#[derive(Iden)]
enum Terms {
    #[iden = "Terms"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Description"]
    Description,
    #[iden = "URL"]
    Url,
    #[iden = "Revision"]
    Revision,
}

#[derive(Iden)]
enum TuVoteInfo {
    #[iden = "TU_VoteInfo"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "Agenda"]
    Agenda,
    #[iden = "User"]
    User,
    #[iden = "Submitted"]
    Submitted,
    #[iden = "End"]
    End,
    #[iden = "Quorum"]
    Quorum,
    #[iden = "SubmitterID"]
    SubmitterId,
    #[iden = "Yes"]
    Yes,
    #[iden = "No"]
    No,
    #[iden = "Abstain"]
    Abstain,
    #[iden = "ActiveTUs"]
    ActiveTus,
}
#[derive(Iden)]
enum TuVotes {
    #[iden = "TU_Votes"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "VoteID"]
    VoteId,
    #[iden = "UserID"]
    UserId,
}

#[derive(Iden)]
enum Users {
    #[iden = "Users"]
    Table,
    #[iden = "ID"]
    Id,
    #[iden = "AccountTypeID"]
    AccountTypeId,
    #[iden = "Suspended"]
    Suspended,
    #[iden = "Username"]
    Username,
    #[iden = "Email"]
    Email,
    #[iden = "BackupEmail"]
    BackupEmail,
    #[iden = "HideEmail"]
    HideEmail,
    #[iden = "Passwd"]
    Passwd,
    #[iden = "Salt"]
    Salt,
    #[iden = "ResetKey"]
    ResetKey,
    #[iden = "RealName"]
    RealName,
    #[iden = "LangPreference"]
    LangPreference,
    #[iden = "Timezone"]
    Timezone,
    #[iden = "Homepage"]
    Homepage,
    #[iden = "IRCNick"]
    IrcNick,
    #[iden = "PGPKey"]
    PgpKey,
    #[iden = "LastLogin"]
    LastLogin,
    #[iden = "LastLoginIPAddress"]
    LastLoginIpAddress,
    #[iden = "LastSSHLogin"]
    LastSshLogin,
    #[iden = "LastSSHLoginIPAddress"]
    LastSshLoginIpAddress,
    #[iden = "InactivityTS"]
    InactivityTs,
    #[iden = "RegistrationTS"]
    RegistrationTs,
    #[iden = "CommentNotify"]
    CommentNotify,
    #[iden = "UpdateNotify"]
    UpdateNotify,
    #[iden = "OwnershipNotify"]
    OwnershipNotify,
    #[iden = "SSOAccountID"]
    SsoAccountId,
}
