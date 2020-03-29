import UserRepository from '@/repository/localstorage/UserRepository';

interface Module {
  userRepository: UserRepository;
}

const localstorageModule: Module = {
  userRepository: new UserRepository()
};

export default localstorageModule;
